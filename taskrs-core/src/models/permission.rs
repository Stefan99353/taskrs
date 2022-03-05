use crate::models::IntoActiveModel;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use taskrs_db::models::permission;
use taskrs_db::sea_orm::ActiveValue;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub group: String,
    pub description: Option<String>,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<permission::Model> for Permission {
    fn from(model: permission::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            group: model.group,
            description: None,
            inserted_at: model.inserted_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionCreate {
    pub name: String,
    pub group: String,
    pub description: Option<String>,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl IntoActiveModel<permission::ActiveModel> for PermissionCreate {
    fn into_active_model(self) -> permission::ActiveModel {
        let mut active_model = permission::ActiveModel {
            name: ActiveValue::Set(self.name),
            group: ActiveValue::Set(self.group),
            description: ActiveValue::Set(self.description),
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
pub struct PermissionUpdate {
    pub id: i32,
    pub name: Option<String>,
    pub group: Option<String>,
    pub description: Option<Option<String>>,
    pub inserted_at: Option<Option<NaiveDateTime>>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

impl IntoActiveModel<permission::ActiveModel> for PermissionUpdate {
    fn into_active_model(self) -> permission::ActiveModel {
        let mut active_model = permission::ActiveModel {
            id: ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(name) = self.name {
            active_model.name = ActiveValue::Set(name);
        }
        if let Some(group) = self.group {
            active_model.group = ActiveValue::Set(group);
        }
        if let Some(description) = self.description {
            active_model.description = ActiveValue::Set(description);
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
