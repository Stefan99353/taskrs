use crate::models::IntoActiveModel;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use taskrs_db::models::refresh_token;
use taskrs_db::sea_orm::ActiveValue;

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

impl From<refresh_token::Model> for RefreshToken {
    fn from(model: refresh_token::Model) -> Self {
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

impl IntoActiveModel<refresh_token::ActiveModel> for RefreshTokenCreate {
    fn into_active_model(self) -> refresh_token::ActiveModel {
        let mut active_model = refresh_token::ActiveModel {
            user_id: ActiveValue::Set(self.user_id),
            token: ActiveValue::Set(self.token),
            iat: ActiveValue::Set(self.iat),
            exp: ActiveValue::Set(self.exp),
            ..Default::default()
        };

        if let Some(inserted_at) = self.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }
        if let Some(updated_at) = self.updated_at {
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

impl IntoActiveModel<refresh_token::ActiveModel> for RefreshTokenUpdate {
    fn into_active_model(self) -> refresh_token::ActiveModel {
        let mut active_model = refresh_token::ActiveModel {
            id: ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(user_id) = self.user_id {
            active_model.user_id = ActiveValue::Set(user_id);
        }
        if let Some(token) = self.token {
            active_model.token = ActiveValue::Set(token);
        }
        if let Some(iat) = self.iat {
            active_model.iat = ActiveValue::Set(iat);
        }
        if let Some(exp) = self.exp {
            active_model.exp = ActiveValue::Set(exp);
        }

        if let Some(inserted_at) = self.inserted_at {
            active_model.inserted_at = ActiveValue::Set(inserted_at);
        }
        if let Some(updated_at) = self.updated_at {
            active_model.updated_at = ActiveValue::Set(updated_at);
        }

        active_model
    }
}
