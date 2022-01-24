use chrono::NaiveDateTime;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub enabled: bool,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<crate::models::user::Model> for User {
    fn from(model: crate::models::user::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            first_name: model.first_name,
            last_name: model.last_name,
            enabled: model.enabled,
            inserted_at: model.inserted_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCreate {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub enabled: bool,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl TryFrom<UserCreate> for crate::models::user::ActiveModel {
    type Error = argon2::Error;

    fn try_from(dto: UserCreate) -> Result<Self, Self::Error> {
        let mut active_model = Self {
            email: ActiveValue::Set(dto.email),
            first_name: ActiveValue::Set(dto.first_name),
            last_name: ActiveValue::Set(dto.last_name),
            enabled: ActiveValue::Set(dto.enabled),
            ..Default::default()
        };

        trace!("Hashing password");
        let password_hash = hash_password(&dto.password)?;
        active_model.password_hash = ActiveValue::Set(password_hash);

        if let Some(inserted_at) = dto.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }
        if let Some(updated_at) = dto.updated_at {
            active_model.updated_at = ActiveValue::Set(Some(updated_at));
        }

        Ok(active_model)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdate {
    pub id: i32,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<Option<String>>,
    pub last_name: Option<Option<String>>,
    pub enabled: Option<bool>,
    pub inserted_at: Option<Option<NaiveDateTime>>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

impl TryFrom<UserUpdate> for crate::models::user::ActiveModel {
    type Error = argon2::Error;

    fn try_from(dto: UserUpdate) -> Result<Self, Self::Error> {
        let mut active_model = Self {
            id: ActiveValue::Set(dto.id),
            ..Default::default()
        };

        if let Some(email) = dto.email {
            active_model.email = ActiveValue::Set(email);
        }
        if let Some(password) = dto.password {
            debug!("Hashing password");
            let password_hash = hash_password(&password)?;
            active_model.password_hash = ActiveValue::Set(password_hash);
        }
        if let Some(first_name) = dto.first_name {
            active_model.first_name = ActiveValue::Set(first_name);
        }
        if let Some(last_name) = dto.last_name {
            active_model.last_name = ActiveValue::Set(last_name);
        }
        if let Some(enabled) = dto.enabled {
            active_model.enabled = ActiveValue::Set(enabled);
        }

        if let Some(inserted_at) = dto.inserted_at {
            active_model.inserted_at = ActiveValue::Set(inserted_at);
        }
        if let Some(updated_at) = dto.updated_at {
            active_model.updated_at = ActiveValue::Set(updated_at);
        }

        Ok(active_model)
    }
}

fn hash_password(pw: &str) -> Result<String, argon2::Error> {
    let salt = rand::random::<[u8; 16]>();
    let config = argon2::Config::default();
    argon2::hash_encoded(pw.as_bytes(), &salt, &config)
}
