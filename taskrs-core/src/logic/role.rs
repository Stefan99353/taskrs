use crate::logic::{
    CreateModelTrait, DeleteModelTrait, PaginatedModelTrait, ReadModelTrait, UpdateModelTrait,
};
use crate::models::permission::Permission;
use crate::models::role::{Role, RoleCreate, RoleUpdate};
use crate::models::role_permission::RolePermission;
use crate::models::user::User;
use crate::models::user_role::UserRole;
use std::collections::HashSet;
use taskrs_db::models::{permission, role, role_permission, user, user_role};
use taskrs_db::sea_orm::prelude::*;
use taskrs_db::sea_orm::sea_query::{IntoCondition, SimpleExpr};
use taskrs_db::sea_orm::{
    Condition, ConnectionTrait, IntoSimpleExpr, JoinType, Order, QuerySelect, TransactionError,
};
use taskrs_db::utils::QueryId;

impl Role {
    pub async fn permissions<'a, C>(role_id: i32, db: &'a C) -> Result<Vec<Permission>, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .join_rev(
                JoinType::InnerJoin,
                role_permission::Relation::Permission.def(),
            )
            .all(db)
            .await
            .map(|models| models.into_iter().map(Permission::from).collect())
    }

    pub async fn grant_permissions<'a, C>(
        role_id: i32,
        permission_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        // Get direct permissions of role
        let current_permission_ids: HashSet<i32> = role_permission::Entity::find()
            .select_only()
            .column_as(role_permission::Column::PermissionId, QueryId::Id)
            .filter(role_permission::Column::RoleId.eq(role_id))
            .into_values::<_, QueryId>()
            .all(db)
            .await?
            .into_iter()
            .collect();

        // Filter only new permissions
        let new_permission_ids: HashSet<i32> = permission_ids.into_iter().collect();
        let role_permissions: Vec<RolePermission> = (&new_permission_ids - &current_permission_ids)
            .into_iter()
            .map(|permission_id| RolePermission {
                role_id,
                permission_id,
                inserted_at: None,
            })
            .collect();

        // Insert models
        if !role_permissions.is_empty() {
            RolePermission::create_many(role_permissions, db).await?;
        }

        Ok(())
    }

    pub async fn revoke_permissions<'a, C>(
        role_id: i32,
        permission_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        RolePermission::delete_condition(
            Condition::all()
                .add(role_permission::Column::RoleId.eq(role_id))
                .add(role_permission::Column::PermissionId.is_in(permission_ids)),
            db,
        )
        .await?;

        Ok(())
    }

    pub async fn set_permissions<'a, C>(
        role_id: i32,
        permission_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), TransactionError<DbErr>>
    where
        C: ConnectionTrait<'a>,
    {
        // Create models for inserting
        let new_role_permissions: Vec<RolePermission> = permission_ids
            .into_iter()
            .map(|permission_id| RolePermission {
                role_id,
                permission_id,
                inserted_at: None,
            })
            .collect();

        // Transaction
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // Delete all role permissions
                RolePermission::delete_condition(
                    role_permission::Column::RoleId.eq(role_id).into_condition(),
                    txn,
                )
                .await?;

                // Insert new permissions
                if !new_role_permissions.is_empty() {
                    RolePermission::create_many(new_role_permissions, txn).await?;
                }

                Ok(())
            })
        })
        .await
    }

    pub async fn users<'a, C>(role_id: i32, db: &'a C) -> Result<Vec<User>, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        user::Entity::find()
            .filter(user_role::Column::RoleId.eq(role_id))
            .join_rev(JoinType::InnerJoin, user_role::Relation::User.def())
            .all(db)
            .await
            .map(|models| models.into_iter().map(User::from).collect())
    }

    pub async fn add_users<'a, C>(role_id: i32, user_ids: Vec<i32>, db: &'a C) -> Result<(), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        // Get users of role
        let current_user_ids: HashSet<i32> = user_role::Entity::find()
            .select_only()
            .column_as(user_role::Column::UserId, QueryId::Id)
            .filter(user_role::Column::RoleId.eq(role_id))
            .into_values::<_, QueryId>()
            .all(db)
            .await?
            .into_iter()
            .collect();

        // Filter only new users
        let new_user_ids: HashSet<i32> = user_ids.into_iter().collect();
        let user_roles: Vec<UserRole> = (&new_user_ids - &current_user_ids)
            .into_iter()
            .map(|user_id| UserRole {
                role_id,
                user_id,
                inserted_at: None,
            })
            .collect();

        // Insert models
        if !user_roles.is_empty() {
            UserRole::create_many(user_roles, db).await?;
        }

        Ok(())
    }

    pub async fn remove_users<'a, C>(
        role_id: i32,
        user_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        UserRole::delete_condition(
            Condition::all()
                .add(user_role::Column::RoleId.eq(role_id))
                .add(user_role::Column::UserId.is_in(user_ids)),
            db,
        )
        .await?;

        Ok(())
    }

    pub async fn set_users<'a, C>(
        role_id: i32,
        user_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), TransactionError<DbErr>>
    where
        C: ConnectionTrait<'a>,
    {
        // Create models for inserting
        let new_user_roles: Vec<UserRole> = user_ids
            .into_iter()
            .map(|user_id| UserRole {
                role_id,
                user_id,
                inserted_at: None,
            })
            .collect();

        // Transaction
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // Delete all user roles
                RolePermission::delete_condition(
                    user_role::Column::RoleId.eq(role_id).into_condition(),
                    txn,
                )
                .await?;

                // Insert new users
                if !new_user_roles.is_empty() {
                    UserRole::create_many(new_user_roles, txn).await?;
                }

                Ok(())
            })
        })
        .await
    }
}

impl CreateModelTrait<role::Entity, role::ActiveModel, RoleCreate> for Role {}

impl ReadModelTrait<role::Entity> for Role {}

impl UpdateModelTrait<role::Entity, role::ActiveModel, RoleUpdate> for Role {}

impl DeleteModelTrait<role::Entity, role::ActiveModel> for Role {}

impl PaginatedModelTrait<role::Entity, role::ActiveModel> for Role {
    fn default_order() -> (SimpleExpr, Order) {
        (role::Column::Id.into_simple_expr(), Order::Asc)
    }
}
