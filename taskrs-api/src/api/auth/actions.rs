use crate::api::error::ApiError;
use crate::config::AuthenticationConfig;
use crate::dtos::login_user::LoginUserDto;
use crate::dtos::token_data::{AccessTokenData, RefreshTokenData};
use serde::Serialize;
use taskrs_db::models::refresh_token::dtos::RefreshTokenCreate;
use taskrs_db::sea_orm::prelude::*;
use taskrs_db::sea_orm::Condition;

pub async fn login(
    login_data: LoginUserDto,
    config: &AuthenticationConfig,
    db: &DbConn,
) -> Result<(String, String), ApiError> {
    debug!("Finding enabled user that matches email");

    let user = taskrs_db::models::user::Entity::find()
        .filter(
            Condition::all()
                .add(taskrs_db::models::user::Column::Email.eq(login_data.email))
                .add(taskrs_db::models::user::Column::Enabled.eq(true)),
        )
        .one(db)
        .await
        .map_err(ApiError::Database)?;

    let user = match user {
        None => {
            return Err(ApiError::WrongCredentials);
        }
        Some(user) => user,
    };

    debug!("Verifying password of user");
    let matches =
        taskrs_db::argon2::verify_encoded(&user.password_hash, login_data.password.as_bytes())
            .map_err(ApiError::Argon)?;
    if !matches {
        trace!("Password does not match hash");
        return Err(ApiError::WrongCredentials);
    }

    debug!("Generating access and refresh tokens");
    let user_id = user.id;
    let access_token_data = AccessTokenData::new(user.into(), config.access_token_expiration_time);
    let refresh_token_data = RefreshTokenData::new(user_id, config.refresh_token_expiration_time);

    let access_token = generate_token(&access_token_data, config.access_token_secret.as_bytes())
        .map_err(ApiError::Jwt)?;
    let refresh_token = generate_token(&refresh_token_data, config.refresh_token_secret.as_bytes())
        .map_err(ApiError::Jwt)?;

    debug!("Saving refresh token");
    let new_token = RefreshTokenCreate {
        user_id,
        token: refresh_token.clone(),
        iat: refresh_token_data.iat,
        exp: refresh_token_data.exp,
        inserted_at: None,
        updated_at: None,
    };
    taskrs_db::actions::refresh_tokens::create(new_token, db)
        .await
        .map_err(ApiError::Database)?;

    Ok((access_token, refresh_token))
}

fn generate_token<T: Serialize>(
    data: &T,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    debug!("Generating token");
    let key = jsonwebtoken::EncodingKey::from_secret(secret);
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), data, &key)
}
