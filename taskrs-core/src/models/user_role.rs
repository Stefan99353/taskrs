use crate::models::IntoActiveModel;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use taskrs_db::models::user_role;
use taskrs_db::sea_orm::ActiveValue;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
    pub inserted_at: Option<NaiveDateTime>,
}

impl From<user_role::Model> for UserRole {
    fn from(model: user_role::Model) -> Self {
        Self {
            user_id: model.user_id,
            role_id: model.role_id,
            inserted_at: model.inserted_at,
        }
    }
}

impl IntoActiveModel<user_role::ActiveModel> for UserRole {
    fn into_active_model(self) -> user_role::ActiveModel {
        let mut active_model = user_role::ActiveModel {
            user_id: ActiveValue::Set(self.user_id),
            role_id: ActiveValue::Set(self.role_id),
            ..Default::default()
        };

        if let Some(inserted_at) = self.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }

        active_model
    }
}
