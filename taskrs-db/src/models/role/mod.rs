pub mod dtos;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub inserted_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "roles"
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
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
            Self::Description => ColumnType::String(None).def().nullable(),
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
    #[cfg(feature = "db-timestamps")]
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        let timestamp = chrono::Utc::now().naive_utc();

        // Inserted timestamp
        if let (&sea_orm::ActiveValue::NotSet, true) = (&self.inserted_at, insert) {
            self.inserted_at = sea_orm::ActiveValue::Set(Some(timestamp));
        }

        // Updated timestamp
        if let sea_orm::ActiveValue::NotSet = self.updated_at {
            self.updated_at = sea_orm::ActiveValue::Set(Some(timestamp));
        }

        Ok(self)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}
