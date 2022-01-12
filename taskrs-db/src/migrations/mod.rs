use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::{
    ActiveValue, ConnectionTrait, DbBackend, ExecResult, Schema, Statement, TransactionError,
};

pub struct Migrations {
    migrations: Vec<Box<dyn Migration>>,
    _target: Option<String>,
}

impl Migrations {
    pub fn new(target: Option<String>) -> Self {
        Self {
            migrations: vec![],
            _target: target,
        }
    }

    pub async fn run(self, db: &DbConn) -> Result<(), TransactionError<DbErr>> {
        create_migrations_table(db)
            .await
            .map_err(TransactionError::Connection)?;

        for migration in self.migrations {
            migration.up(db).await?;
        }

        Ok(())
    }
}

async fn create_migrations_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let schema = Schema::new(db.get_database_backend());
    let mut stmt = schema.create_table_from_entity(migration::Entity);
    stmt.if_not_exists();

    db.execute(db.get_database_backend().build(&stmt)).await
}

/// Trait provides necessary functionality to create database migrations
#[async_trait]
pub trait Migration: Sync {
    /// Defines the execution order of all migration
    /// `1` comes first, then `2`, etc.
    fn order(&self) -> u32;
    /// Defines the name of the migration
    fn name(&self) -> String;
    /// Creates all statements necessary to execute migration
    fn up_statements(&self, backend: DbBackend) -> Vec<Statement>;
    /// Creates all statements necessary to rollback migration
    fn down_statements(&self, _backend: DbBackend) -> Vec<Statement> {
        vec![]
    }

    /// Executes the migration
    async fn up(&self, db: &DbConn) -> Result<(), TransactionError<DbErr>> {
        let name = self.name();
        let order = self.order();
        let statements = self.up_statements(db.get_database_backend());

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                if migration::Entity::find_by_id(name.clone())
                    .one(txn)
                    .await?
                    .is_some()
                {
                    // Migration is already present
                    return Ok(());
                }

                // Run statements
                for statement in statements {
                    txn.execute(statement).await?;
                }

                // Insert migration
                migration::ActiveModel {
                    name: ActiveValue::Set(name),
                    order: ActiveValue::Set(order),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }

    /// Rolls back the migration
    async fn down(&self, db: &DbConn) -> Result<(), TransactionError<DbErr>> {
        let name = self.name();
        let statements = self.down_statements(db.get_database_backend());

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                if let Some(db_mig) = migration::Entity::find_by_id(name.clone()).one(txn).await? {
                    // Run statements
                    for statement in statements {
                        txn.execute(statement).await?;
                    }

                    // Remove migration
                    db_mig.delete(txn).await?;
                }

                Ok(())
            })
        })
        .await
    }
}

pub mod migration {
    use sea_orm::entity::prelude::*;
    use sea_orm::ActiveValue;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize, DeriveEntityModel)]
    #[serde(rename_all = "camelCase")]
    #[sea_orm(table_name = "_migrations")]
    pub struct Model {
        #[sea_orm(primary_key, unique, auto_increment = false)]
        pub name: String,
        pub order: u32,
        pub run_at: DateTime,
    }

    #[derive(Debug, Copy, Clone, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {
        fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
            self.run_at = ActiveValue::Set(chrono::Utc::now().naive_utc());
            Ok(self)
        }
    }
}
