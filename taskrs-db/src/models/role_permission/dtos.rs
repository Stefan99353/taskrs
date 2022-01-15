use chrono::NaiveDateTime;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
    pub inserted_at: Option<NaiveDateTime>,
}

impl From<crate::models::role_permission::Model> for RolePermission {
    fn from(model: crate::models::role_permission::Model) -> Self {
        Self {
            role_id: model.role_id,
            permission_id: model.permission_id,
            inserted_at: model.inserted_at
        }
    }
}

impl From<RolePermission> for crate::models::role_permission::ActiveModel {
    fn from(dto: RolePermission) -> Self {
        let mut active_model = Self {
            role_id: ActiveValue::Set(dto.role_id),
            permission_id: ActiveValue::Set(dto.permission_id),
            ..Default::default()
        };

        if let Some(inserted_at) = dto.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }

        active_model
    }
}