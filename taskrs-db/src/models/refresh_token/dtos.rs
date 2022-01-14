use chrono::NaiveDateTime;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub iat: i64,
    pub exp: i64,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<crate::models::refresh_token::Model> for RefreshToken {
    fn from(model: crate::models::refresh_token::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            token: model.token,
            iat: model.iat,
            exp: model.exp,
            inserted_at: model.inserted_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenCreate {
    pub user_id: i32,
    pub token: String,
    pub iat: i64,
    pub exp: i64,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<RefreshTokenCreate> for crate::models::refresh_token::ActiveModel {
    fn from(dto: RefreshTokenCreate) -> Self {
        let mut active_model = Self {
            user_id: ActiveValue::Set(dto.user_id),
            token: ActiveValue::Set(dto.token),
            iat: ActiveValue::Set(dto.iat),
            exp: ActiveValue::Set(dto.exp),
            ..Default::default()
        };

        if let Some(inserted_at) = dto.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }
        if let Some(updated_at) = dto.updated_at {
            active_model.updated_at = ActiveValue::Set(Some(updated_at));
        }

        active_model
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenUpdate {
    pub id: i32,
    pub user_id: Option<i32>,
    pub token: Option<String>,
    pub iat: Option<i64>,
    pub exp: Option<i64>,
    pub inserted_at: Option<Option<NaiveDateTime>>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

impl From<RefreshTokenUpdate> for crate::models::refresh_token::ActiveModel {
    fn from(dto: RefreshTokenUpdate) -> Self {
        let mut active_model = Self {
            id: ActiveValue::Set(dto.id),
            ..Default::default()
        };

        if let Some(user_id) = dto.user_id {
            active_model.user_id = ActiveValue::Set(user_id);
        }
        if let Some(token) = dto.token {
            active_model.token = ActiveValue::Set(token);
        }
        if let Some(iat) = dto.iat {
            active_model.iat = ActiveValue::Set(iat);
        }
        if let Some(exp) = dto.exp {
            active_model.exp = ActiveValue::Set(exp);
        }

        if let Some(inserted_at) = dto.inserted_at {
            active_model.inserted_at = ActiveValue::Set(inserted_at);
        }
        if let Some(updated_at) = dto.updated_at {
            active_model.updated_at = ActiveValue::Set(updated_at);
        }

        active_model
    }
}
