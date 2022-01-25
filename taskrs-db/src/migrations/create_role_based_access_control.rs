use crate::migrations::Migration;
use crate::models::{permission, role, role_permission, user_permission, user_role};
use async_trait::async_trait;
use sea_orm::sea_query::Table;
use sea_orm::{DbBackend, Schema, Statement};

#[derive(Default)]
pub(crate) struct CreateRoleBasedAccessControlMigration;

#[async_trait]
impl Migration for CreateRoleBasedAccessControlMigration {
    fn order(&self) -> u32 {
        20
    }

    fn name(&self) -> String {
        String::from("create_role_based_access_control")
    }

    fn up_statements(&self, backend: DbBackend) -> Vec<Statement> {
        let schema = Schema::new(backend);
        let mut statements = vec![];

        // Permissions
        let permissions_stmt = schema.create_table_from_entity(permission::Entity);
        statements.push(backend.build(&permissions_stmt));

        // Roles
        let roles_stmt = schema.create_table_from_entity(role::Entity);
        statements.push(backend.build(&roles_stmt));

        // UserPermissions
        let user_permissions_stmt = schema.create_table_from_entity(user_permission::Entity);
        statements.push(backend.build(&user_permissions_stmt));

        // RolePermissions
        let role_permissions_stmt = schema.create_table_from_entity(role_permission::Entity);
        statements.push(backend.build(&role_permissions_stmt));

        // UserRoles
        let user_roles_stmt = schema.create_table_from_entity(user_role::Entity);
        statements.push(backend.build(&user_roles_stmt));

        statements
    }

    fn down_statements(&self, backend: DbBackend) -> Vec<Statement> {
        let mut statements = vec![];

        // UserRoles
        let user_roles_stmt = Table::drop().table(user_role::Entity).to_owned();
        statements.push(backend.build(&user_roles_stmt));

        // RolePermissions
        let role_permissions_stmt = Table::drop().table(role_permission::Entity).to_owned();
        statements.push(backend.build(&role_permissions_stmt));

        // UserPermissions
        let user_permissions_stmt = Table::drop().table(user_permission::Entity).to_owned();
        statements.push(backend.build(&user_permissions_stmt));

        // Roles
        let roles_stmt = Table::drop().table(role::Entity).to_owned();
        statements.push(backend.build(&roles_stmt));

        // Permissions
        let permissions_stmt = Table::drop().table(permission::Entity).to_owned();
        statements.push(backend.build(&permissions_stmt));

        statements
    }
}
