use crate::models::role;
use crate::models::role::dtos::{Role, RoleCreate, RoleUpdate};
use futures::try_join;
use sea_orm::prelude::*;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{Condition, ConnectionTrait, Order, QueryOrder};

/// Gets all roles from database
#[instrument(
    name = "get_all_roles"
    level = "debug",
    skip_all,
    fields (
        condition = tracing::field::debug(&condition),
        order = tracing::field::debug(&order),
    )
)]
pub async fn get_all<'a, C>(
    condition: Option<Condition>,
    order: Option<Vec<(Order, SimpleExpr)>>,
    db: &'a C,
) -> Result<Vec<Role>, DbErr>
where
    C: ConnectionTrait<'a>,
{
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
    fields (
        page = page,
        limit = limit,
        condition = tracing::field::debug(&condition),
        order = tracing::field::debug(&order),
    )
)]
pub async fn get_paginated<'a, C>(
    page: usize,
    limit: usize,
    condition: Option<Condition>,
    order: Option<Vec<(Order, SimpleExpr)>>,
    db: &'a C,
) -> Result<(Vec<Role>, usize), DbErr>
where
    C: ConnectionTrait<'a>,
{
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
    fields (
        id = tracing::field::debug(id),
        condition = tracing::field::debug(&condition),
    )
)]
pub async fn get<'a, C>(
    id: Option<i32>,
    condition: Option<Condition>,
    db: &'a C,
) -> Result<Option<Role>, DbErr>
where
    C: ConnectionTrait<'a>,
{
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
)]
pub async fn create<'a, C>(role: RoleCreate, db: &'a C) -> Result<Role, DbErr>
where
    C: ConnectionTrait<'a>,
{
    let active_model: role::ActiveModel = role.into();
    debug!("Inserting new permission");
    active_model.insert(db).await.map(|model| model.into())
}

/// Create new roles and returns last inserted id
#[instrument(
name = "create_roles"
level = "debug",
skip_all,
)]
pub async fn create_many<'a, C>(roles: Vec<RoleCreate>, db: &'a C) -> Result<i32, DbErr>
where
    C: ConnectionTrait<'a>,
{
    let active_models: Vec<role::ActiveModel> = roles.into_iter().map(|p| p.into()).collect();

    debug!("Inserting new roles");
    role::Entity::insert_many(active_models)
        .exec(db)
        .await
        .map(|r| r.last_insert_id)
}

/// Updates a role
#[instrument(
    name = "update_role"
    level = "debug",
    skip_all,
    fields (
        id = role.id
    )
)]
pub async fn update<'a, C>(role: RoleUpdate, db: &'a C) -> Result<Role, DbErr>
where
    C: ConnectionTrait<'a>,
{
    let active_model: role::ActiveModel = role.into();
    debug!("Updating permission");
    active_model.insert(db).await.map(|model| model.into())
}

/// Deletes a role
#[instrument(
    name = "delete_roles"
    level = "debug",
    skip_all,
    fields (
        id = tracing::field::debug(id),
        condition = tracing::field::debug(&condition),
    )
)]
pub async fn delete<'a, C>(
    id: Option<i32>,
    condition: Option<Condition>,
    db: &'a C,
) -> Result<u64, DbErr>
where
    C: ConnectionTrait<'a>,
{
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
