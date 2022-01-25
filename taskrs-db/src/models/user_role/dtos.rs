use chrono::NaiveDateTime;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
    pub inserted_at: Option<NaiveDateTime>,
}

impl From<crate::models::user_role::Model> for UserRole {
    fn from(model: crate::models::user_role::Model) -> Self {
        Self {
            user_id: model.user_id,
            role_id: model.role_id,
            inserted_at: model.inserted_at,
        }
    }
}

impl From<UserRole> for crate::models::user_role::ActiveModel {
    fn from(dto: UserRole) -> Self {
        let mut active_model = Self {
            user_id: ActiveValue::Set(dto.user_id),
            role_id: ActiveValue::Set(dto.role_id),
            ..Default::default()
        };

        if let Some(inserted_at) = dto.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }

        active_model
    }
}
