use crate::models::IntoActiveModel;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use taskrs_db::models::user_permission;
use taskrs_db::sea_orm::ActiveValue;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPermission {
    pub user_id: i32,
    pub permission_id: i32,
    pub inserted_at: Option<NaiveDateTime>,
}

impl From<user_permission::Model> for UserPermission {
    fn from(model: user_permission::Model) -> Self {
        Self {
            user_id: model.user_id,
            permission_id: model.permission_id,
            inserted_at: model.inserted_at,
        }
    }
}

impl IntoActiveModel<user_permission::ActiveModel> for UserPermission {
    fn into_active_model(self) -> user_permission::ActiveModel {
        let mut active_model = user_permission::ActiveModel {
            user_id: ActiveValue::Set(self.user_id),
            permission_id: ActiveValue::Set(self.permission_id),
            ..Default::default()
        };

        if let Some(inserted_at) = self.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }

        active_model
    }
}
