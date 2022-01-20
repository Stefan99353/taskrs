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

// User Permissions
pub async fn grant_user_permissions(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), DbErr> {
    // Get permission ids of user
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
    user_permission::Entity::insert_many(active_models)
        .exec(db)
        .await?;

    Ok(())
}

pub async fn revoke_user_permissions(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), DbErr> {
    // Delete permissions of user
    user_permission::Entity::delete_many()
        .filter(user_permission::Column::UserId.eq(user_id))
        .filter(user_permission::Column::PermissionId.is_in(permission_ids))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn set_user_permissions(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), TransactionError<DbErr>> {
    // Create active models for inserting
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
            // Delete all permissions of user
            user_permission::Entity::delete_many()
                .filter(user_permission::Column::UserId.eq(user_id))
                .exec(txn)
                .await?;

            // Insert new permissions
            user_permission::Entity::insert_many(active_models)
                .exec(txn)
                .await?;

            Ok(())
        })
    })
    .await
}

// Role Permissions
pub async fn grant_role_permissions(
    role_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), DbErr> {
    // Get permission ids of role
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
    role_permission::Entity::insert_many(active_models)
        .exec(db)
        .await?;

    Ok(())
}

pub async fn revoke_role_permissions(
    role_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), DbErr> {
    // Delete permissions of role
    role_permission::Entity::delete_many()
        .filter(role_permission::Column::RoleId.eq(role_id))
        .filter(role_permission::Column::PermissionId.is_in(permission_ids))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn set_role_permissions(
    role_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), TransactionError<DbErr>> {
    // Create active models for inserting
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
            // Delete all permissions of role
            role_permission::Entity::delete_many()
                .filter(role_permission::Column::RoleId.eq(role_id))
                .exec(txn)
                .await?;

            // Insert new permissions
            role_permission::Entity::insert_many(active_models)
                .exec(txn)
                .await?;

            Ok(())
        })
    })
    .await
}

// User Roles
pub async fn add_user_roles(user_id: i32, role_ids: Vec<i32>, db: &DbConn) -> Result<(), DbErr> {
    // Get role ids of user
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
    user_role::Entity::insert_many(active_models)
        .exec(db)
        .await?;

    Ok(())
}

pub async fn remove_user_roles(user_id: i32, role_ids: Vec<i32>, db: &DbConn) -> Result<(), DbErr> {
    // Delete roles of user
    user_role::Entity::delete_many()
        .filter(user_role::Column::UserId.eq(user_id))
        .filter(user_role::Column::RoleId.is_in(role_ids))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn set_user_roles(
    user_id: i32,
    role_ids: Vec<i32>,
    db: &DbConn,
) -> Result<(), TransactionError<DbErr>> {
    // Create active models for inserting
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
            // Delete all roles of user
            user_role::Entity::delete_many()
                .filter(user_role::Column::UserId.eq(user_id))
                .exec(txn)
                .await?;

            // Insert new roles
            user_role::Entity::insert_many(active_models)
                .exec(txn)
                .await?;

            Ok(())
        })
    })
    .await
}

pub async fn get_permissions_of_user(user_id: i32, db: &DbConn) -> Result<Vec<Permission>, DbErr> {
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

pub async fn get_permission_of_role(role_id: i32, db: &DbConn) -> Result<Vec<Permission>, DbErr> {
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

pub async fn get_roles_of_user(user_id: i32, db: &DbConn) -> Result<Vec<Role>, DbErr> {
    role::Entity::find()
        .filter(user_role::Column::UserId.eq(user_id))
        .join_rev(JoinType::InnerJoin, user_role::Relation::Role.def())
        .all(db)
        .await
        .map(|models| models.into_iter().map(|model| model.into()).collect())
}

pub async fn get_all_permission_of_user(
    user_id: i32,
    db: &DbConn,
) -> Result<Vec<Permission>, DbErr> {
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

pub async fn has_one_permission(
    user_id: i32,
    permission_id: i32,
    db: &DbConn,
) -> Result<bool, DbErr> {
    get_all_permission_of_user(user_id, db)
        .await
        .map(|permissions| permissions.iter().any(|p| p.id == permission_id))
}

pub async fn has_any_permission(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<bool, DbErr> {
    let all_permissions = get_all_permission_of_user(user_id, db).await?;

    for per in all_permissions {
        if permission_ids.iter().any(|p| p == &per.id) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub async fn has_all_permissions(
    user_id: i32,
    permission_ids: Vec<i32>,
    db: &DbConn,
) -> Result<bool, DbErr> {
    let all_permissions = get_all_permission_of_user(user_id, db).await?;

    for id in permission_ids {
        if !all_permissions.iter().any(|p| p.id == id) {
            return Ok(false);
        }
    }

    Ok(true)
}
