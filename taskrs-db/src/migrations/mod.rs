mod create_refresh_tokens;
mod create_role_based_access_control;
mod create_users;

use async_trait::async_trait;
use create_refresh_tokens::CreateRefreshTokensMigration;
use create_role_based_access_control::CreateRoleBasedAccessControlMigration;
use create_users::CreateUsersMigration;
use itertools::Itertools;
use sea_orm::prelude::*;
use sea_orm::{
    ActiveValue, ConnectionTrait, DbBackend, ExecResult, QueryOrder, Schema, Statement,
    TransactionError,
};

/// Used to perform migrations.
/// Use `target` to update database to a specific migration.
pub struct Migrations {
    migrations: Vec<Box<dyn Migration>>,
    target: Option<String>,
}

impl Migrations {
    pub fn new(target: Option<String>) -> Self {
        Self {
            migrations: vec![
                Box::new(CreateUsersMigration),
                Box::new(CreateRefreshTokensMigration),
                Box::new(CreateRoleBasedAccessControlMigration),
            ],
            target,
        }
    }

    /// Runs the migrations. If a target is present, tries to update database to that specific migration.
    #[instrument(name = "migrations_run", level = "debug", skip_all, fields(target))]
    pub async fn run(self, db: &DbConn) -> Result<(), MigrationError> {
        debug!("Creating migrations table if it does not exist");
        create_migrations_table(db)
            .await
            .map_err(|e| MigrationError::Db(TransactionError::Connection(e)))?;

        trace!("Sort migrations by their order property");
        let ordered_migrations = self
            .migrations
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.order(), &b.order()));

        if let Some(target) = self.target {
            // Update/Downgrade to specific migration
            tracing::Span::current().record("target", &tracing::field::display(&target));
            debug!("Updating database to specific migrations");
            Migrations::run_to_target(ordered_migrations.collect(), target, db).await?;
        } else {
            // Update to latest migration
            debug!("Updating database to latest migration");
            Migrations::check_and_up(ordered_migrations.collect(), db).await?;
        }

        Ok(())
    }

    async fn check_and_up(
        migrations: Vec<Box<dyn Migration>>,
        db: &DbConn,
    ) -> Result<(), MigrationError> {
        // Get already run migrations
        debug!("Get already run migrations to skip them");
        let db_migrations = migration::Entity::find()
            .order_by_asc(migration::Column::Order)
            .all(db)
            .await
            .map_err(|e| MigrationError::Db(TransactionError::Connection(e)))?;

        debug!("Running remaining migrations");
        for migration in migrations {
            if !db_migrations.iter().any(|m| m.name == migration.name()) {
                migration.up(db).await.map_err(MigrationError::Db)?;
            }
        }

        Ok(())
    }

    async fn run_to_target(
        migrations: Vec<Box<dyn Migration>>,
        target: String,
        db: &DbConn,
    ) -> Result<(), MigrationError> {
        // Check if target is a valid
        debug!("Check if provided target is a known migration");
        let target_order =
            if let Some(target_migration) = migrations.iter().find(|m| m.name() == target) {
                target_migration.order()
            } else {
                // Target is no valid migration
                return Err(MigrationError::TargetInvalid);
            };

        // Get already run migrations
        debug!("Retrieve database migration state to determine if need to upgrade or downgrade");
        let db_migrations = migration::Entity::find()
            .order_by_asc(migration::Column::Order)
            .all(db)
            .await
            .map_err(|e| MigrationError::Db(TransactionError::Connection(e)))?;

        // Check if need to down or up
        if db_migrations.iter().any(|m| m.name == target) {
            // Downgrade
            debug!("Downgrade to target migration");
            let down_migration_names: Vec<String> = db_migrations
                .iter()
                .filter(|m| m.order > target_order)
                .map(|m| m.name.clone())
                .collect();

            let down_migrations = migrations
                .iter()
                .filter(|m| down_migration_names.iter().any(|dm| dm == &m.name()));

            for down_migration in down_migrations.rev() {
                down_migration.down(db).await.map_err(MigrationError::Db)?;
            }
        } else {
            // Upgrade
            debug!("Upgrade to target migration");
            let up_migrations = migrations.into_iter().filter(|m| m.order() <= target_order);
            Migrations::check_and_up(up_migrations.collect(), db).await?;
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

#[derive(Debug)]
pub enum MigrationError {
    Db(TransactionError<DbErr>),
    TargetInvalid,
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationError::Db(e) => std::fmt::Display::fmt(e, f),
            MigrationError::TargetInvalid => write!(f, "Provided target migration is not valid"),
        }
    }
}

impl std::error::Error for MigrationError {}

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
    #[instrument(
        name = "migration_up",
        level = "debug",
        skip_all,
        fields(
            name = %self.name(),
            order = self.order(),
        )
    )]
    async fn up(&self, db: &DbConn) -> Result<(), TransactionError<DbErr>> {
        let name = self.name();
        let order = self.order();
        let statements = self.up_statements(db.get_database_backend());

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                debug!("Started transaction");

                debug!("Check if migration already exists");
                if migration::Entity::find_by_id(name.clone())
                    .one(txn)
                    .await?
                    .is_some()
                {
                    // Migration is already present
                    debug!("Migration already exists. Skipping");
                    return Ok(());
                }

                // Run statements
                debug!("Execute up statements");
                for statement in statements {
                    txn.execute(statement).await?;
                }

                // Insert migration
                debug!("Insert migration into migrations table");
                migration::ActiveModel {
                    name: ActiveValue::Set(name),
                    order: ActiveValue::Set(order),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                debug!("Migration successful");
                Ok(())
            })
        })
        .await
    }

    /// Rolls back the migration
    #[instrument(
        name = "migration_down",
        level = "debug",
        skip_all,
        fields(
            name = %self.name(),
        )
    )]
    async fn down(&self, db: &DbConn) -> Result<(), TransactionError<DbErr>> {
        let name = self.name();
        let statements = self.down_statements(db.get_database_backend());

        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                debug!("Started transaction");

                debug!("Check if migration exists");
                if let Some(db_mig) = migration::Entity::find_by_id(name.clone()).one(txn).await? {
                    // Run statements
                    debug!("Execute down statements");
                    for statement in statements {
                        txn.execute(statement).await?;
                    }

                    // Remove migration
                    debug!("Remove migration from migrations table");
                    db_mig.delete(txn).await?;
                }

                debug!("Migration successful");
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
