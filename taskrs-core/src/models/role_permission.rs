use crate::models::IntoActiveModel;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use taskrs_db::models::role_permission;
use taskrs_db::sea_orm::ActiveValue;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
    pub inserted_at: Option<NaiveDateTime>,
}

impl From<role_permission::Model> for RolePermission {
    fn from(model: role_permission::Model) -> Self {
        Self {
            role_id: model.role_id,
            permission_id: model.permission_id,
            inserted_at: model.inserted_at,
        }
    }
}

impl IntoActiveModel<role_permission::ActiveModel> for RolePermission {
    fn into_active_model(self) -> role_permission::ActiveModel {
        let mut active_model = role_permission::ActiveModel {
            role_id: ActiveValue::Set(self.role_id),
            permission_id: ActiveValue::Set(self.permission_id),
            ..Default::default()
        };

        if let Some(inserted_at) = self.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }

        active_model
    }
}
