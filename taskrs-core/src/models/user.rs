use crate::models::IntoActiveModel;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use taskrs_db::models::user;
use taskrs_db::sea_orm::ActiveValue;

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

impl From<user::Model> for User {
    fn from(model: user::Model) -> Self {
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

impl UserCreate {
    pub fn hash_password(mut self) -> Result<Self, argon2::Error> {
        let salt = rand::random::<[u8; 16]>();
        let config = argon2::Config::default();
        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)?;
        Ok(self)
    }
}

impl IntoActiveModel<user::ActiveModel> for UserCreate {
    fn into_active_model(self) -> user::ActiveModel {
        let mut active_model = user::ActiveModel {
            email: ActiveValue::Set(self.email),
            password_hash: ActiveValue::Set(self.password),
            first_name: ActiveValue::Set(self.first_name),
            last_name: ActiveValue::Set(self.last_name),
            enabled: ActiveValue::Set(self.enabled),
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

impl UserUpdate {
    pub fn hash_password(mut self) -> Result<Self, argon2::Error> {
        if let Some(password) = &self.password {
            let salt = rand::random::<[u8; 16]>();
            let config = argon2::Config::default();
            self.password = Some(argon2::hash_encoded(password.as_bytes(), &salt, &config)?);
        }

        Ok(self)
    }
}

impl IntoActiveModel<user::ActiveModel> for UserUpdate {
    fn into_active_model(self) -> user::ActiveModel {
        let mut active_model = user::ActiveModel {
            id: ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(email) = self.email {
            active_model.email = ActiveValue::Set(email);
        }
        if let Some(password) = self.password {
            active_model.password_hash = ActiveValue::Set(password);
        }
        if let Some(first_name) = self.first_name {
            active_model.first_name = ActiveValue::Set(first_name);
        }
        if let Some(last_name) = self.last_name {
            active_model.last_name = ActiveValue::Set(last_name);
        }
        if let Some(enabled) = self.enabled {
            active_model.enabled = ActiveValue::Set(enabled);
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
