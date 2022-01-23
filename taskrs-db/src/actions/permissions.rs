use crate::models::permission;
use crate::models::permission::dtos::{Permission, PermissionCreate, PermissionUpdate};
use futures::try_join;
use sea_orm::prelude::*;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{Condition, Order, QueryOrder};

/// Gets all permissions from database
#[instrument(
    name = "get_all_permissions"
    level = "debug",
    skip_all,
    err,
    fields (
        condition = tracing::field::debug(&condition),
        order = tracing::field::debug(&order),
    )
)]
pub async fn get_all(
    condition: Option<Condition>,
    order: Option<Vec<(Order, SimpleExpr)>>,
    db: &DbConn,
) -> Result<Vec<Permission>, DbErr> {
    let mut query = permission::Entity::find();

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    if let Some(order) = order {
        trace!("Apply order to query");
        for (ord, col) in order {
            query = query.order_by(col, ord);
        }
    }

    debug!("Executing query");
    query
        .all(db)
        .await
        .map(|models| models.into_iter().map(|x| x.into()).collect())
}

/// Gets all permissions from database in a paginated form
#[instrument(
    name = "get_paginated_permissions"
    level = "debug",
    skip_all,
    err,
    fields (
        page = page,
        limit = limit,
        condition = tracing::field::debug(&condition),
        order = tracing::field::debug(&order),
    )
)]
pub async fn get_paginated(
    page: usize,
    limit: usize,
    condition: Option<Condition>,
    order: Option<Vec<(Order, SimpleExpr)>>,
    db: &DbConn,
) -> Result<(Vec<Permission>, usize), DbErr> {
    let mut query = permission::Entity::find();

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    if let Some(order) = order {
        trace!("Apply order to query");
        for (ord, col) in order {
            query = query.order_by(col, ord);
        }
    }

    debug!("Executing query and getting count of items");
    let paginator = query.paginate(db, limit);
    let (models, count) = try_join!(paginator.fetch_page(page), paginator.num_items())?;

    Ok((models.into_iter().map(|x| x.into()).collect(), count))
}

/// Gets a single permission from the database using an ID and/or a condition
#[instrument(
    name = "get_permission"
    level = "debug",
    skip_all,
    err,
    fields (
        id = tracing::field::debug(id),
        condition = tracing::field::debug(&condition),
    )
)]
pub async fn get(
    id: Option<i32>,
    condition: Option<Condition>,
    db: &DbConn,
) -> Result<Option<Permission>, DbErr> {
    let mut query = if let Some(id) = id {
        permission::Entity::find_by_id(id)
    } else {
        permission::Entity::find()
    };

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    debug!("Executing query");
    query.one(db).await.map(|opt| opt.map(|model| model.into()))
}

/// Creates a new permission
#[instrument(
    name = "create_permission"
    level = "debug",
    skip_all,
    err,
)]
pub async fn create(permission: PermissionCreate, db: &DbConn) -> Result<Permission, DbErr> {
    let active_model: permission::ActiveModel = permission.into();
    debug!("Inserting new permission");
    active_model.insert(db).await.map(|model| model.into())
}

/// Updates a permission
#[instrument(
    name = "update_permission"
    level = "debug",
    skip_all,
    err,
    fields (
        id = permission.id
    )
)]
pub async fn update(permission: PermissionUpdate, db: &DbConn) -> Result<Permission, DbErr> {
    let active_model: permission::ActiveModel = permission.into();
    debug!("Updating permission");
    active_model.insert(db).await.map(|model| model.into())
}

/// Deletes a permission
#[instrument(
    name = "delete_permissions"
    level = "debug",
    skip_all,
    err,
    fields (
        id = tracing::field::debug(id),
        condition = tracing::field::debug(&condition),
    )
)]
pub async fn delete(
    id: Option<i32>,
    condition: Option<Condition>,
    db: &DbConn,
) -> Result<u64, DbErr> {
    let mut query = permission::Entity::delete_many();

    if let Some(id) = id {
        trace!("Filter for ID");
        query = query.filter(permission::Column::Id.eq(id))
    }

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    debug!("Executing query");
    query.exec(db).await.map(|res| res.rows_affected)
}
