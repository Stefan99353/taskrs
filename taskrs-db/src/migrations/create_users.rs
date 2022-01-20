use crate::migrations::Migration;
use crate::models::user;
use async_trait::async_trait;
use sea_orm::{DbBackend, Schema, Statement};

#[derive(Default)]
pub(crate) struct CreateUsersMigration;

#[async_trait]
impl Migration for CreateUsersMigration {
    fn order(&self) -> u32 {
        0
    }

    fn name(&self) -> String {
        String::from("create_users")
    }

    fn up_statements(&self, backend: DbBackend) -> Vec<Statement> {
        let schema = Schema::new(backend);
        let mut stmt = schema.create_table_from_entity(user::Entity);
        stmt.if_not_exists();

        vec![backend.build(&stmt)]
    }
}
