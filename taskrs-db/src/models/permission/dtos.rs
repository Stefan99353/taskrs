use chrono::NaiveDateTime;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

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

impl From<crate::models::permission::Model> for Permission {
    fn from(model: crate::models::permission::Model) -> Self {
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

impl From<PermissionCreate> for crate::models::permission::ActiveModel {
    fn from(dto: PermissionCreate) -> Self {
        let mut active_model = Self {
            name: ActiveValue::Set(dto.name),
            group: ActiveValue::Set(dto.group),
            description: ActiveValue::Set(dto.description),
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
pub struct PermissionUpdate {
    pub id: i32,
    pub name: Option<String>,
    pub group: Option<String>,
    pub description: Option<Option<String>>,
    pub inserted_at: Option<Option<NaiveDateTime>>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

impl From<PermissionUpdate> for crate::models::permission::ActiveModel {
    fn from(dto: PermissionUpdate) -> Self {
        let mut active_model = Self {
            id: ActiveValue::Set(dto.id),
            ..Default::default()
        };

        if let Some(name) = dto.name {
            active_model.name = ActiveValue::Set(name);
        }
        if let Some(group) = dto.group {
            active_model.group = ActiveValue::Set(group);
        }
        if let Some(description) = dto.description {
            active_model.description = ActiveValue::Set(description);
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