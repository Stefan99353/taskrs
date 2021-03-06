use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, DeriveModel, DeriveActiveModel)]
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
            Self::InsertedAt => ColumnType::DateTime.def().nullable(),
            Self::UpdatedAt => ColumnType::DateTime.def().nullable(),
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
    #[cfg(feature = "db-timestamps")]
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        let timestamp = chrono::Utc::now().naive_utc();

        // Inserted timestamp
        if let (&sea_orm::ActiveValue::NotSet, true) = (&self.inserted_at, insert) {
            trace!("Setting inserted_at timestamp for user");
            self.inserted_at = sea_orm::ActiveValue::Set(Some(timestamp));
        }

        // Updated timestamp
        if let sea_orm::ActiveValue::NotSet = self.updated_at {
            trace!("Setting updated_at timestamp for user");
            self.updated_at = sea_orm::ActiveValue::Set(Some(timestamp));
        }

        Ok(self)
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Permission,
    RefreshToken,
    Role,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Permission => Entity::has_many(crate::models::permission::Entity).into(),
            Self::RefreshToken => Entity::has_many(crate::models::refresh_token::Entity).into(),
            Self::Role => Entity::has_many(crate::models::role::Entity).into(),
        }
    }
}

impl Related<crate::models::permission::Entity> for Entity {
    fn to() -> RelationDef {
        crate::models::user_permission::Relation::Permission.def()
    }

    fn via() -> Option<RelationDef> {
        Some(crate::models::user_permission::Relation::User.def().rev())
    }
}

impl Related<crate::models::refresh_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshToken.def()
    }
}

impl Related<crate::models::role::Entity> for Entity {
    fn to() -> RelationDef {
        crate::models::user_role::Relation::Role.def()
    }

    fn via() -> Option<RelationDef> {
        Some(crate::models::user_role::Relation::User.def().rev())
    }
}
