use crate::models::user::User;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Default)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessTokenData {
    pub iat: i64,
    pub exp: i64,
    pub user: User,
}

impl AccessTokenData {
    pub fn new(user: User, exp: u32) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + exp as i64,
            user,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RefreshTokenData {
    pub iat: i64,
    pub exp: i64,
    pub user_id: i32,
}

impl RefreshTokenData {
    pub fn new(user_id: i32, exp: u32) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + exp as i64,
            user_id,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AuthSettings {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub access_token_expiration_time: u32,  // Seconds
    pub refresh_token_expiration_time: u32, // Seconds
}
