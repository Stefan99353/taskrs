use chrono::NaiveDateTime;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPermission {
    pub user_id: i32,
    pub permission_id: i32,
    pub inserted_at: Option<NaiveDateTime>,
}

impl From<crate::models::user_permission::Model> for UserPermission {
    fn from(model: crate::models::user_permission::Model) -> Self {
        Self {
            user_id: model.user_id,
            permission_id: model.permission_id,
            inserted_at: model.inserted_at
        }
    }
}

impl From<UserPermission> for crate::models::user_permission::ActiveModel {
    fn from(dto: UserPermission) -> Self {
        let mut active_model = Self {
            user_id: ActiveValue::Set(dto.user_id),
            permission_id: ActiveValue::Set(dto.permission_id),
            ..Default::default()
        };

        if let Some(inserted_at) = dto.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }

        active_model
    }
}