use crate::migrations::Migration;
use crate::models::refresh_token;
use async_trait::async_trait;
use sea_orm::{DbBackend, Schema, Statement};

#[derive(Default)]
pub(crate) struct CreateRefreshTokensMigration;

#[async_trait]
impl Migration for CreateRefreshTokensMigration {
    fn order(&self) -> u32 {
        10
    }

    fn name(&self) -> String {
        String::from("create_refresh_tokens")
    }

    fn up_statements(&self, backend: DbBackend) -> Vec<Statement> {
        let schema = Schema::new(backend);
        let mut stmt = schema.create_table_from_entity(refresh_token::Entity);
        stmt.if_not_exists();

        vec![backend.build(&stmt)]
    }
}
