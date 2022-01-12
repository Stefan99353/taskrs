use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

#[derive(Clone, Debug, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub enabled: bool,
    pub inserted_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users"
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Email,
    PasswordHash,
    FirstName,
    LastName,
    Enabled,
    InsertedAt,
    UpdatedAt,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Email => ColumnType::String(Some(256)).def().unique(),
            Self::PasswordHash => ColumnType::String(Some(1024)).def(),
            Self::FirstName => ColumnType::String(Some(256)).def().nullable(),
            Self::LastName => ColumnType::String(Some(256)).def().nullable(),
            Self::Enabled => ColumnType::Boolean.def(),
            Self::InsertedAt => ColumnType::DateTime.def(),
            Self::UpdatedAt => ColumnType::DateTime.def(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        true
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        let timestamp = chrono::Utc::now().naive_utc();
        self.updated_at = ActiveValue::Set(Some(timestamp));

        if insert {
            self.inserted_at = ActiveValue::Set(Some(timestamp));
        }

        Ok(self)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
