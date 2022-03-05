use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub iat: i64,
    pub exp: i64,
    pub inserted_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "refresh_tokens"
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    UserId,
    Token,
    Iat,
    Exp,
    InsertedAt,
    UpdatedAt,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::UserId => ColumnType::Integer.def(),
            Self::Token => ColumnType::String(None).def(),
            Self::Iat => ColumnType::BigInteger.def(),
            Self::Exp => ColumnType::BigInteger.def(),
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
            trace!("Setting inserted_at timestamp for refresh_token");
            self.inserted_at = sea_orm::ActiveValue::Set(Some(timestamp));
        }

        // Updated timestamp
        if let sea_orm::ActiveValue::NotSet = self.updated_at {
            trace!("Setting updated_at timestamp for refresh_token");
            self.updated_at = sea_orm::ActiveValue::Set(Some(timestamp));
        }

        Ok(self)
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(crate::models::user::Entity)
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

impl Related<crate::models::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
