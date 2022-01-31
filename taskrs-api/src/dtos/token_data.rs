use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessTokenData {
    pub iat: i64,
    pub exp: i64,
    pub user: taskrs_db::models::user::dtos::User,
}

impl AccessTokenData {
    pub fn new(user: taskrs_db::models::user::dtos::User, exp: u32) -> Self {
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
