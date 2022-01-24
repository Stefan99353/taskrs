use crate::models::role;
use crate::models::role::dtos::{Role, RoleCreate, RoleUpdate};
use futures::try_join;
use sea_orm::prelude::*;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{Condition, Order, QueryOrder};

/// Gets all roles from database
#[instrument(
    name = "get_all_roles"
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
) -> Result<Vec<Role>, DbErr> {
    let mut query = role::Entity::find();

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

/// Gets all roles from database in a paginated form
#[instrument(
    name = "get_paginated_roles"
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
) -> Result<(Vec<Role>, usize), DbErr> {
    let mut query = role::Entity::find();

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    if let Some(order) = order {
        for (ord, col) in order {
            trace!("Apply order to query");
            query = query.order_by(col, ord);
        }
    }

    debug!("Executing query and getting count of items");
    let paginator = query.paginate(db, limit);
    let (models, count) = try_join!(paginator.fetch_page(page), paginator.num_items())?;

    Ok((models.into_iter().map(|x| x.into()).collect(), count))
}

/// Gets a single role from the database using an ID and/or a condition
#[instrument(
    name = "get_role"
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
) -> Result<Option<Role>, DbErr> {
    let mut query = if let Some(id) = id {
        role::Entity::find_by_id(id)
    } else {
        role::Entity::find()
    };

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    debug!("Executing query");
    query.one(db).await.map(|opt| opt.map(|model| model.into()))
}

/// Creates a new role
#[instrument(
    name = "create_role"
    level = "debug",
    skip_all,
    err,
)]
pub async fn create(role: RoleCreate, db: &DbConn) -> Result<Role, DbErr> {
    let active_model: role::ActiveModel = role.into();
    debug!("Inserting new permission");
    active_model.insert(db).await.map(|model| model.into())
}

/// Updates a role
#[instrument(
    name = "update_role"
    level = "debug",
    skip_all,
    err,
    fields (
        id = role.id
    )
)]
pub async fn update(role: RoleUpdate, db: &DbConn) -> Result<Role, DbErr> {
    let active_model: role::ActiveModel = role.into();
    debug!("Updating permission");
    active_model.insert(db).await.map(|model| model.into())
}

/// Deletes a role
#[instrument(
    name = "delete_roles"
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
    let mut query = role::Entity::delete_many();

    if let Some(id) = id {
        trace!("Filter for ID");
        query = query.filter(role::Column::Id.eq(id))
    }

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    debug!("Executing query");
    query.exec(db).await.map(|res| res.rows_affected)
}
