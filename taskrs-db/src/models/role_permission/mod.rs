pub mod dtos;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub role_id: i32,
    pub permission_id: i32,
    pub inserted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "role_permissions"
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    RoleId,
    PermissionId,
    InsertedAt,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::RoleId => ColumnType::Integer.def(),
            Self::PermissionId => ColumnType::Integer.def(),
            Self::InsertedAt => ColumnType::DateTime.def().nullable(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    RoleId,
    PermissionId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32);

    fn auto_increment() -> bool {
        false
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

        Ok(self)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
