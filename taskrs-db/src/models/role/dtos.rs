use chrono::NaiveDateTime;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<crate::models::role::Model> for Role {
    fn from(model: crate::models::role::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: None,
            inserted_at: model.inserted_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleCreate {
    pub name: String,
    pub description: Option<String>,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<RoleCreate> for crate::models::role::ActiveModel {
    fn from(dto: RoleCreate) -> Self {
        let mut active_model = Self {
            name: ActiveValue::Set(dto.name),
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
pub struct RoleUpdate {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub inserted_at: Option<Option<NaiveDateTime>>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

impl From<RoleUpdate> for crate::models::role::ActiveModel {
    fn from(dto: RoleUpdate) -> Self {
        let mut active_model = Self {
            id: ActiveValue::Set(dto.id),
            ..Default::default()
        };

        if let Some(name) = dto.name {
            active_model.name = ActiveValue::Set(name);
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