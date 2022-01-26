use crate::models::permission::dtos::Permission;
use crate::models::role::dtos::Role;
use crate::models::{permission, role, role_permission, user, user_permission, user_role};
use crate::utils::QueryId;
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::sea_query::Query;
use sea_orm::JoinType;
use sea_orm::{ActiveValue, Condition, ConnectionTrait, QuerySelect, TransactionError};
use std::collections::HashSet;

/// Grant permissions to user. Only grants new permissions.
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
        permission_ids = tracing::field::debug(&permission_ids),
    )
)]
pub async fn grant_user_permissions<'a, C>(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &'a C,
) -> Result<(), DbErr>
where
    C: ConnectionTrait<'a>,
{
    // Get permission ids of user
    debug!("Get current permissions of user");
    let old_permission_ids: HashSet<i32> = user_permission::Entity::find()
        .select_only()
        .column(user_permission::Column::PermissionId)
        .filter(user_permission::Column::UserId.eq(user_id))
        .into_values::<_, QueryId>()
        .all(db)
        .await?
        .into_iter()
        .collect();

    // Get difference and create active models for inserting
    trace!("Filter out permissions the user already has");
    let new_permission_ids: HashSet<i32> = permission_ids.into_iter().collect();
    let active_models =
        (&new_permission_ids - &old_permission_ids)
            .into_iter()
            .map(|permission_id| user_permission::ActiveModel {
                user_id: ActiveValue::Set(user_id),
                permission_id: ActiveValue::Set(permission_id),
                ..Default::default()
            });

    // Insert models
    debug!("Inserting new permissions");
    user_permission::Entity::insert_many(active_models)
        .exec(db)
        .await?;

    Ok(())
}

/// Revoke permissions from user.
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
        permission_ids = tracing::field::debug(&permission_ids),
    )
)]
pub async fn revoke_user_permissions<'a, C>(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &'a C,
) -> Result<(), DbErr>
where
    C: ConnectionTrait<'a>,
{
    // Delete permissions of user
    debug!("Removing permissions");
    user_permission::Entity::delete_many()
        .filter(user_permission::Column::UserId.eq(user_id))
        .filter(user_permission::Column::PermissionId.is_in(permission_ids))
        .exec(db)
        .await?;

    Ok(())
}

/// Set permissions for user.
/// Cannot be used in a transaction, as this action uses a transaction itself
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
        permission_ids = tracing::field::debug(&permission_ids),
    )
)]
pub async fn set_user_permissions(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), TransactionError<DbErr>> {
    // Create active models for inserting
    trace!("Creating models for inserting to keep transaction short");
    let active_models: Vec<user_permission::ActiveModel> = permission_ids
        .iter()
        .map(|permission_id| user_permission::ActiveModel {
            user_id: ActiveValue::Set(user_id),
            permission_id: ActiveValue::Set(*permission_id),
            ..Default::default()
        })
        .collect();

    // Execute in transaction
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            debug!("Starting transaction");

            // Delete all permissions of user
            debug!("Removing all permissions from user");
            user_permission::Entity::delete_many()
                .filter(user_permission::Column::UserId.eq(user_id))
                .exec(txn)
                .await?;

            // Insert new permissions
            debug!("Inserting new permissions for user");
            user_permission::Entity::insert_many(active_models)
                .exec(txn)
                .await?;

            debug!("Setting permissions successful");
            Ok(())
        })
    })
    .await
}

/// Grant permissions to role. Only grants new permissions.
#[instrument(
    level = "debug",
    skip_all,
    fields (
        role_id = role_id,
        permission_ids = tracing::field::debug(&permission_ids),
    )
)]
pub async fn grant_role_permissions<'a, C>(
    role_id: i32,
    permission_ids: Vec<i32>,
    db: &'a C,
) -> Result<(), DbErr>
where
    C: ConnectionTrait<'a>,
{
    // Get permission ids of role
    debug!("Get current permissions of role");
    let old_permission_ids: HashSet<i32> = role_permission::Entity::find()
        .select_only()
        .column(role_permission::Column::PermissionId)
        .filter(role_permission::Column::RoleId.eq(role_id))
        .into_values::<_, QueryId>()
        .all(db)
        .await?
        .into_iter()
        .collect();

    // Get difference and create active models for inserting
    trace!("Filter out permissions the role already has");
    let new_permission_ids: HashSet<i32> = permission_ids.into_iter().collect();
    let active_models =
        (&new_permission_ids - &old_permission_ids)
            .into_iter()
            .map(|permission_id| role_permission::ActiveModel {
                role_id: ActiveValue::Set(role_id),
                permission_id: ActiveValue::Set(permission_id),
                ..Default::default()
            });

    // Insert models
    debug!("Inserting new permissions");
    role_permission::Entity::insert_many(active_models)
        .exec(db)
        .await?;

    Ok(())
}

/// Revoke permissions from role.
#[instrument(
    level = "debug",
    skip_all,
    fields (
        role_id = role_id,
        permission_ids = tracing::field::debug(&permission_ids),
    )
)]
pub async fn revoke_role_permissions<'a, C>(
    role_id: i32,
    permission_ids: Vec<i32>,
    db: &'a C,
) -> Result<(), DbErr>
where
    C: ConnectionTrait<'a>,
{
    // Delete permissions of role
    debug!("Removing permissions");
    role_permission::Entity::delete_many()
        .filter(role_permission::Column::RoleId.eq(role_id))
        .filter(role_permission::Column::PermissionId.is_in(permission_ids))
        .exec(db)
        .await?;

    Ok(())
}

/// Set permissions for role.
/// Cannot be used in a transaction, as this action uses a transaction itself
#[instrument(
    level = "debug",
    skip_all,
    fields (
        role_id = role_id,
        permission_ids = tracing::field::debug(&permission_ids),
    )
)]
pub async fn set_role_permissions(
    role_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), TransactionError<DbErr>> {
    // Create active models for inserting
    trace!("Creating models for inserting to keep transaction short");
    let active_models: Vec<role_permission::ActiveModel> = permission_ids
        .iter()
        .map(|permission_id| role_permission::ActiveModel {
            role_id: ActiveValue::Set(role_id),
            permission_id: ActiveValue::Set(*permission_id),
            ..Default::default()
        })
        .collect();

    // Execute in transaction
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            debug!("Starting transaction");

            // Delete all permissions of role
            debug!("Removing all permissions from role");
            role_permission::Entity::delete_many()
                .filter(role_permission::Column::RoleId.eq(role_id))
                .exec(txn)
                .await?;

            // Insert new permissions
            debug!("Inserting new permissions for role");
            role_permission::Entity::insert_many(active_models)
                .exec(txn)
                .await?;

            debug!("Setting permissions successful");
            Ok(())
        })
    })
    .await
}

/// Add user to roles. Only adds user to new roles.
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
        role_ids = tracing::field::debug(&role_ids),
    )
)]
pub async fn add_user_roles<'a, C>(user_id: i32, role_ids: Vec<i32>, db: &'a C) -> Result<(), DbErr>
where
    C: ConnectionTrait<'a>,
{
    // Get role ids of user
    debug!("Get current roles of user");
    let old_role_ids: HashSet<i32> = user_role::Entity::find()
        .select_only()
        .column(user_role::Column::RoleId)
        .filter(user_role::Column::UserId.eq(user_id))
        .into_values::<_, QueryId>()
        .all(db)
        .await?
        .into_iter()
        .collect();

    // Get difference and create active models for inserting
    trace!("Filter out roles the user already has");
    let new_role_ids: HashSet<i32> = role_ids.into_iter().collect();
    let active_models =
        (&new_role_ids - &old_role_ids)
            .into_iter()
            .map(|role_id| user_role::ActiveModel {
                user_id: ActiveValue::Set(user_id),
                role_id: ActiveValue::Set(role_id),
                ..Default::default()
            });

    // Insert models
    debug!("Inserting new roles");
    user_role::Entity::insert_many(active_models)
        .exec(db)
        .await?;

    Ok(())
}

/// Remove roles from user.
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
        role_ids = tracing::field::debug(&role_ids),
    )
)]
pub async fn remove_user_roles<'a, C>(
    user_id: i32,
    role_ids: Vec<i32>,
    db: &'a C,
) -> Result<(), DbErr>
where
    C: ConnectionTrait<'a>,
{
    // Delete roles of user
    debug!("Removing roles");
    user_role::Entity::delete_many()
        .filter(user_role::Column::UserId.eq(user_id))
        .filter(user_role::Column::RoleId.is_in(role_ids))
        .exec(db)
        .await?;

    Ok(())
}

/// Set roles for user.
/// Cannot be used in a transaction, as this action uses a transaction itself
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
        role_ids = tracing::field::debug(&role_ids),
    )
)]
pub async fn set_user_roles(
    user_id: i32,
    role_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), TransactionError<DbErr>> {
    // Create active models for inserting
    trace!("Creating models for inserting to keep transaction short");
    let active_models: Vec<user_role::ActiveModel> = role_ids
        .iter()
        .map(|role_id| user_role::ActiveModel {
            user_id: ActiveValue::Set(user_id),
            role_id: ActiveValue::Set(*role_id),
            ..Default::default()
        })
        .collect();

    // Execute in transaction
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            debug!("Starting transaction");

            // Delete all roles of user
            debug!("Removing all roles from user");
            user_role::Entity::delete_many()
                .filter(user_role::Column::UserId.eq(user_id))
                .exec(txn)
                .await?;

            // Insert new roles
            debug!("Inserting new permissions for role");
            user_role::Entity::insert_many(active_models)
                .exec(txn)
                .await?;

            debug!("Setting roles successful");
            Ok(())
        })
    })
    .await
}

/// Get permissions of user. Only gets direct permissions (Without roles).
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
    )
)]
pub async fn get_permissions_of_user<'a, C>(
    user_id: i32,
    db: &'a C,
) -> Result<Vec<Permission>, DbErr>
where
    C: ConnectionTrait<'a>,
{
    debug!("Getting all direct permissions of user");
    permission::Entity::find()
        .filter(user_permission::Column::UserId.eq(user_id))
        .join_rev(
            JoinType::InnerJoin,
            user_permission::Relation::Permission.def(),
        )
        .all(db)
        .await
        .map(|models| models.into_iter().map(|model| model.into()).collect())
}

/// Get permissions of role.
#[instrument(
    level = "debug",
    skip_all,
    fields (
        role_id = role_id,
    )
)]
pub async fn get_permission_of_role<'a, C>(
    role_id: i32,
    db: &'a C,
) -> Result<Vec<Permission>, DbErr>
where
    C: ConnectionTrait<'a>,
{
    debug!("Getting all permissions of role");
    permission::Entity::find()
        .filter(role_permission::Column::RoleId.eq(role_id))
        .join_rev(
            JoinType::InnerJoin,
            role_permission::Relation::Permission.def(),
        )
        .all(db)
        .await
        .map(|models| models.into_iter().map(|model| model.into()).collect())
}

/// Get roles of user.
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
    )
)]
pub async fn get_roles_of_user<'a, C>(user_id: i32, db: &'a C) -> Result<Vec<Role>, DbErr>
where
    C: ConnectionTrait<'a>,
{
    debug!("Getting all roles of user");
    role::Entity::find()
        .filter(user_role::Column::UserId.eq(user_id))
        .join_rev(JoinType::InnerJoin, user_role::Relation::Role.def())
        .all(db)
        .await
        .map(|models| models.into_iter().map(|model| model.into()).collect())
}

/// Get all permissions a user has. Includes inherited permissions of roles.
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
    )
)]
pub async fn get_all_permission_of_user<'a, C>(
    user_id: i32,
    db: &'a C,
) -> Result<Vec<Permission>, DbErr>
where
    C: ConnectionTrait<'a>,
{
    // SELECT *
    //     FROM permissions
    // WHERE id IN (
    //     SELECT p.id
    //     FROM users u
    //     INNER JOIN user_permissions up on u.id = up.user_id
    //     INNER JOIN user_roles ur on u.id = ur.user_id
    //     INNER JOIN roles r on r.id = ur.role_id
    //     INNER JOIN role_permissions rp on r.id = rp.role_id
    //     INNER JOIN permissions p on p.id = up.permission_id OR p.id = rp.permission_id
    //     WHERE u.id = 1
    // )

    debug!("Getting all permissions of user");
    permission::Entity::find()
        .filter(
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
        .all(db)
        .await
        .map(|models| models.into_iter().map(|model| model.into()).collect())
}

/// Check if a user has a permission
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
        permission_id = permission_id,
    )
)]
pub async fn has_one_permission<'a, C>(
    user_id: i32,
    permission_id: i32,
    db: &'a C,
) -> Result<bool, DbErr>
where
    C: ConnectionTrait<'a>,
{
    let all_permissions = get_all_permission_of_user(user_id, db).await;

    debug!("Checking if permission is included");
    all_permissions.map(|permissions| permissions.iter().any(|p| p.id == permission_id))
}

/// Check if a user has one of multiple permissions
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
        permission_ids = tracing::field::debug(&permission_ids),
    )
)]
pub async fn has_any_permission<'a, C>(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &'a C,
) -> Result<bool, DbErr>
where
    C: ConnectionTrait<'a>,
{
    let all_permissions = get_all_permission_of_user(user_id, db).await?;

    debug!("Checking if one permission is included");
    for per in all_permissions {
        if permission_ids.iter().any(|p| p == &per.id) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Check if a user has all of multiple permissions
#[instrument(
    level = "debug",
    skip_all,
    fields (
        user_id = user_id,
        permission_ids = tracing::field::debug(&permission_ids),
    )
)]
pub async fn has_all_permissions<'a, C>(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &'a C,
) -> Result<bool, DbErr>
where
    C: ConnectionTrait<'a>,
{
    let all_permissions = get_all_permission_of_user(user_id, db).await?;

    debug!("Checking if all permission are included");
    for id in permission_ids {
        if !all_permissions.iter().any(|p| p.id == id) {
            return Ok(false);
        }
    }

    Ok(true)
}
