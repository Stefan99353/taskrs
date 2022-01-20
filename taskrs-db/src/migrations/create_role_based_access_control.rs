use crate::migrations::Migration;
use crate::models::{permission, role, role_permission, user_permission, user_role};
use async_trait::async_trait;
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
        let mut permissions_stmt = schema.create_table_from_entity(permission::Entity);
        permissions_stmt.if_not_exists();
        statements.push(backend.build(&permissions_stmt));

        // Roles
        let mut roles_stmt = schema.create_table_from_entity(role::Entity);
        roles_stmt.if_not_exists();
        statements.push(backend.build(&roles_stmt));

        // UserPermissions
        let mut user_permissions_stmt = schema.create_table_from_entity(user_permission::Entity);
        user_permissions_stmt.if_not_exists();
        statements.push(backend.build(&user_permissions_stmt));

        // RolePermissions
        let mut role_permissions_stmt = schema.create_table_from_entity(role_permission::Entity);
        role_permissions_stmt.if_not_exists();
        statements.push(backend.build(&role_permissions_stmt));

        // UserRoles
        let mut user_roles_stmt = schema.create_table_from_entity(user_role::Entity);
        user_roles_stmt.if_not_exists();
        statements.push(backend.build(&user_roles_stmt));

        statements
    }
}
