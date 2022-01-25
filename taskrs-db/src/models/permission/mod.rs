pub mod dtos;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub group: String,
    pub description: Option<String>,
    pub inserted_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "permissions"
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Group,
    Description,
    InsertedAt,
    UpdatedAt,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(Some(256)).def(),
            Self::Group => ColumnType::String(Some(256)).def(),
            Self::Description => ColumnType::String(None).def().nullable(),
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
            trace!("Setting inserted_at timestamp for permission");
            self.inserted_at = sea_orm::ActiveValue::Set(Some(timestamp));
        }

        // Updated timestamp
        if let sea_orm::ActiveValue::NotSet = self.updated_at {
            trace!("Setting updated_at timestamp for permission");
            self.updated_at = sea_orm::ActiveValue::Set(Some(timestamp));
        }

        Ok(self)
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Role,
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Role => Entity::has_many(crate::models::role::Entity).into(),
            Self::User => Entity::has_many(crate::models::user::Entity).into(),
        }
    }
}

impl Related<crate::models::role::Entity> for Entity {
    fn to() -> RelationDef {
        crate::models::role_permission::Relation::Role.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            crate::models::role_permission::Relation::Permission
                .def()
                .rev(),
        )
    }
}

impl Related<crate::models::user::Entity> for Entity {
    fn to() -> RelationDef {
        crate::models::user_permission::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            crate::models::user_permission::Relation::Permission
                .def()
                .rev(),
        )
    }
}
