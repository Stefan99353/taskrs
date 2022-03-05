use crate::logic::{
    CreateModelTrait, DeleteModelTrait, PaginatedModelTrait, ReadModelTrait, UpdateModelTrait,
};
use crate::models::permission::Permission;
use crate::models::role::Role;
use crate::models::user::{User, UserCreate, UserUpdate};
use crate::models::user_permission::UserPermission;
use crate::models::user_role::UserRole;
use std::collections::HashSet;
use taskrs_db::models::{permission, role, role_permission, user, user_permission, user_role};
use taskrs_db::sea_orm::prelude::*;
use taskrs_db::sea_orm::sea_query::{Expr, IntoCondition, Query, SimpleExpr};
use taskrs_db::sea_orm::{Condition, ConnectionTrait, JoinType, QuerySelect, TransactionError};
use taskrs_db::sea_orm::{IntoSimpleExpr, Order};
use taskrs_db::utils::QueryId;

impl User {
    /// Checks if an user with email already exists
    pub async fn email_exists<'a, C>(email: &str, db: &'a C) -> Result<bool, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
            .map(|res| res.is_some())
    }

    pub async fn has_one_permission<'a, C>(
        user_id: i32,
        permission_id: i32,
        db: &'a C,
    ) -> Result<bool, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        Self::permissions_query(user_id)
            .filter(permission::Column::Id.eq(permission_id))
            .one(db)
            .await
            .map(|model| model.is_some())
    }

    pub async fn has_any_permission<'a, C>(
        user_id: i32,
        permission_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<bool, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        Self::permissions_query(user_id)
            .filter(permission::Column::Id.is_in(permission_ids))
            .one(db)
            .await
            .map(|model| model.is_some())
    }

    pub async fn has_all_permissions<'a, C>(
        user_id: i32,
        permission_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<bool, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        let total = permission_ids.len();

        Self::permissions_query(user_id)
            .filter(permission::Column::Id.is_in(permission_ids))
            .all(db)
            .await
            .map(|models| models.len() == total)
    }

    pub async fn permissions<'a, C>(user_id: i32, db: &'a C) -> Result<Vec<Permission>, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        Self::permissions_query(user_id)
            .all(db)
            .await
            .map(|models| models.into_iter().map(Permission::from).collect())
    }

    pub async fn grant_permissions<'a, C>(
        user_id: i32,
        permission_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        // Get direct permissions of user
        let current_permission_ids: HashSet<i32> = user_permission::Entity::find()
            .select_only()
            .column_as(user_permission::Column::PermissionId, QueryId::Id)
            .filter(user_permission::Column::UserId.eq(user_id))
            .into_values::<_, QueryId>()
            .all(db)
            .await?
            .into_iter()
            .collect();

        // Filter only new permissions
        let new_permission_ids: HashSet<i32> = permission_ids.into_iter().collect();
        let user_permissions: Vec<UserPermission> = (&new_permission_ids - &current_permission_ids)
            .into_iter()
            .map(|permission_id| UserPermission {
                user_id,
                permission_id,
                inserted_at: None,
            })
            .collect();

        // Insert models
        if !user_permissions.is_empty() {
            UserPermission::create_many(user_permissions, db).await?;
        }

        Ok(())
    }

    pub async fn revoke_permissions<'a, C>(
        user_id: i32,
        permission_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        UserPermission::delete_condition(
            Condition::all()
                .add(user_permission::Column::UserId.eq(user_id))
                .add(user_permission::Column::PermissionId.is_in(permission_ids)),
            db,
        )
        .await?;

        Ok(())
    }

    pub async fn set_permissions<'a, C>(
        user_id: i32,
        permission_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), TransactionError<DbErr>>
    where
        C: ConnectionTrait<'a>,
    {
        // Create models for inserting
        let new_user_permissions: Vec<UserPermission> = permission_ids
            .into_iter()
            .map(|permission_id| UserPermission {
                user_id,
                permission_id,
                inserted_at: None,
            })
            .collect();

        // Transaction
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // Delete all user permissions
                UserPermission::delete_condition(
                    user_permission::Column::UserId.eq(user_id).into_condition(),
                    txn,
                )
                .await?;

                // Insert new permissions
                if !new_user_permissions.is_empty() {
                    UserPermission::create_many(new_user_permissions, txn).await?;
                }

                Ok(())
            })
        })
        .await
    }

    pub async fn roles<'a, C>(user_id: i32, db: &'a C) -> Result<Vec<Role>, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .join_rev(JoinType::InnerJoin, user_role::Relation::Role.def())
            .all(db)
            .await
            .map(|models| models.into_iter().map(Role::from).collect())
    }

    pub async fn grant_roles<'a, C>(
        user_id: i32,
        role_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        // Get direct roles of user
        let current_role_ids: HashSet<i32> = user_role::Entity::find()
            .select_only()
            .column_as(user_role::Column::RoleId, QueryId::Id)
            .filter(user_role::Column::UserId.eq(user_id))
            .into_values::<_, QueryId>()
            .all(db)
            .await?
            .into_iter()
            .collect();

        // Filter only new roles
        let new_role_ids: HashSet<i32> = role_ids.into_iter().collect();
        let user_roles: Vec<UserRole> = (&new_role_ids - &current_role_ids)
            .into_iter()
            .map(|role_id| UserRole {
                user_id,
                role_id,
                inserted_at: None,
            })
            .collect();

        // Insert models
        if !user_roles.is_empty() {
            UserRole::create_many(user_roles, db).await?;
        }

        Ok(())
    }

    pub async fn revoke_roles<'a, C>(
        user_id: i32,
        role_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        UserRole::delete_condition(
            Condition::all()
                .add(user_role::Column::UserId.eq(user_id))
                .add(user_role::Column::RoleId.is_in(role_ids)),
            db,
        )
        .await?;

        Ok(())
    }

    pub async fn set_roles<'a, C>(
        user_id: i32,
        role_ids: Vec<i32>,
        db: &'a C,
    ) -> Result<(), TransactionError<DbErr>>
    where
        C: ConnectionTrait<'a>,
    {
        // Create models for inserting
        let new_user_roles: Vec<UserRole> = role_ids
            .into_iter()
            .map(|role_id| UserRole {
                user_id,
                role_id,
                inserted_at: None,
            })
            .collect();

        // Transaction
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // Delete all user roles
                UserRole::delete_condition(
                    user_role::Column::UserId.eq(user_id).into_condition(),
                    txn,
                )
                .await?;

                // Insert new roles
                if !new_user_roles.is_empty() {
                    UserRole::create_many(new_user_roles, txn).await?;
                }

                Ok(())
            })
        })
        .await
    }

    fn permissions_query(user_id: i32) -> Select<permission::Entity> {
        permission::Entity::find().filter(
            permission::Column::Id.in_subquery(
                Query::select()
                    .column((permission::Entity, permission::Column::Id))
                    .from(user::Entity)
                    .inner_join(
                        user_permission::Entity,
                        Expr::tbl(user::Entity, user::Column::Id)
                            .equals(user_permission::Entity, user_permission::Column::UserId),
                    )
                    .inner_join(
                        user_role::Entity,
                        Expr::tbl(user::Entity, user::Column::Id)
                            .equals(user_role::Entity, user_role::Column::UserId),
                    )
                    .inner_join(
                        role::Entity,
                        Expr::tbl(user_role::Entity, user_role::Column::RoleId)
                            .equals(role::Entity, role::Column::Id),
                    )
                    .inner_join(
                        role_permission::Entity,
                        Expr::tbl(role::Entity, role::Column::Id)
                            .equals(role_permission::Entity, role_permission::Column::RoleId),
                    )
                    .inner_join(
                        permission::Entity,
                        Condition::any()
                            .add(
                                Expr::tbl(
                                    user_permission::Entity,
                                    user_permission::Column::PermissionId,
                                )
                                .equals(permission::Entity, permission::Column::Id),
                            )
                            .add(
                                Expr::tbl(
                                    role_permission::Entity,
                                    role_permission::Column::PermissionId,
                                )
                                .equals(permission::Entity, permission::Column::Id),
                            ),
                    )
                    .and_where(user::Column::Id.eq(user_id))
                    .to_owned(),
            ),
        )
    }
}

impl CreateModelTrait<user::Entity, user::ActiveModel, UserCreate> for User {}

impl ReadModelTrait<user::Entity> for User {}

impl UpdateModelTrait<user::Entity, user::ActiveModel, UserUpdate> for User {}

impl DeleteModelTrait<user::Entity, user::ActiveModel> for User {}

impl PaginatedModelTrait<user::Entity, user::ActiveModel> for User {
    fn default_order() -> (SimpleExpr, Order) {
        (user::Column::Id.into_simple_expr(), Order::Asc)
    }
}
