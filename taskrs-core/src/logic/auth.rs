use crate::error::{AuthError, Error};
use crate::logic::{CreateModelTrait, DeleteModelTrait};
use crate::models::auth::{AccessTokenData, Auth, AuthSettings, AuthTokens, RefreshTokenData};
use crate::models::refresh_token::{RefreshToken, RefreshTokenCreate};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::TokenData;
use serde::de::DeserializeOwned;
use serde::Serialize;
use taskrs_db::sea_orm::sea_query::IntoCondition;
use taskrs_db::sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};

impl Auth {
    pub async fn login(self, settings: &AuthSettings, db: &DbConn) -> Result<AuthTokens, Error> {
        debug!("Check if user is available for login");
        let user = taskrs_db::models::user::Entity::find()
            .filter(taskrs_db::models::user::Column::Email.eq(self.email))
            .one(db)
            .await
            .map_err(Error::Database)?;

        let user = user.ok_or(Error::Auth(AuthError::UnknownEmail))?;
        if !user.enabled {
            return Err(Error::Auth(AuthError::UserDisabled));
        }

        debug!("Verify password of user");
        let matches = argon2::verify_encoded(&user.password_hash, self.password.as_bytes())
            .map_err(Error::Argon)?;
        if !matches {
            return Err(Error::Auth(AuthError::WrongPassword));
        }

        debug!("Generating tokens");
        let user_id = user.id;
        let access_token_data =
            AccessTokenData::new(user.into(), settings.access_token_expiration_time);
        let refresh_token_data =
            RefreshTokenData::new(user_id, settings.refresh_token_expiration_time);

        let access_token =
            encode_token(&access_token_data, settings.access_token_secret.as_bytes())
                .map_err(Error::JsonWebToken)?;
        let refresh_token = encode_token(
            &refresh_token_data,
            settings.refresh_token_secret.as_bytes(),
        )
        .map_err(Error::JsonWebToken)?;

        debug!("Saving refresh token");
        RefreshToken::create(
            RefreshTokenCreate {
                user_id,
                token: refresh_token.clone(),
                iat: refresh_token_data.iat,
                exp: refresh_token_data.exp,
                ..Default::default()
            },
            db,
        )
        .await
        .map_err(Error::Database)?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
        })
    }

    pub async fn logout(refresh_token: &str, db: &DbConn) -> Result<u64, Error> {
        RefreshToken::delete_condition(
            taskrs_db::models::refresh_token::Column::Token
                .eq(refresh_token)
                .into_condition(),
            db,
        )
        .await
        .map(|r| r.rows_affected)
        .map_err(Error::Database)
    }
}

pub fn validate_token<T: DeserializeOwned>(
    token: &str,
    secret: &[u8],
) -> (Result<TokenData<T>, jsonwebtoken::errors::Error>, bool) {
    let result = jsonwebtoken::decode::<T>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret),
        &jsonwebtoken::Validation::default(),
    );

    let mut expired = false;

    if let Err(err) = &result {
        let kind = err.kind();
        expired = matches!(kind, &ErrorKind::ExpiredSignature);
    }

    (result, expired)
}

pub fn encode_token<T: Serialize>(
    data: &T,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    let key = jsonwebtoken::EncodingKey::from_secret(secret);
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), data, &key)
}
