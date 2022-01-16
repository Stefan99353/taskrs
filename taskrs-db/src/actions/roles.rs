use futures::try_join;
use sea_orm::{IntoSimpleExpr, Order, QueryOrder};
use sea_orm::prelude::*;
use crate::sea_query::IntoCondition;
use crate::models::role;
use crate::models::role::dtos::{Role, RoleCreate, RoleUpdate};

/// Gets all roles from database
pub async fn get_all<F, C>(
    condition: Option<F>,
    order: Option<Vec<(Order, C)>>,
    db: &DbConn,
) -> Result<Vec<Role>, DbErr>
    where
        F: IntoCondition,
        C: IntoSimpleExpr,
{
    let mut query = role::Entity::find();

    if let Some(condition) = condition {
        query = query.filter(condition);
    }

    if let Some(order) = order {
        for (ord, col) in order {
            query = query.order_by(col, ord);
        }
    }

    query
        .all(db)
        .await
        .map(|models| models.into_iter().map(|x| x.into()).collect())
}

/// Gets all roles from database in a paginated form
pub async fn get_paginated<F, C>(
    page: usize,
    limit: usize,
    condition: Option<F>,
    order: Option<Vec<(Order, C)>>,
    db: &DbConn,
) -> Result<(Vec<Role>, usize), DbErr>
    where
        F: IntoCondition,
        C: IntoSimpleExpr,
{
    let mut query = role::Entity::find();

    if let Some(condition) = condition {
        query = query.filter(condition);
    }

    if let Some(order) = order {
        for (ord, col) in order {
            query = query.order_by(col, ord);
        }
    }

    let paginator = query.paginate(db, limit);
    let (models, count) = try_join!(paginator.fetch_page(page), paginator.num_items())?;

    Ok((models.into_iter().map(|x| x.into()).collect(), count))
}

/// Gets a single role from the database using an ID and/or a condition
pub async fn get<F>(
    id: Option<i32>,
    condition: Option<F>,
    db: &DbConn,
) -> Result<Option<Role>, DbErr>
    where
        F: IntoCondition,
{
    let mut query = if let Some(id) = id {
        role::Entity::find_by_id(id)
    } else {
        role::Entity::find()
    };

    if let Some(condition) = condition {
        query = query.filter(condition);
    }

    query.one(db).await.map(|opt| opt.map(|model| model.into()))
}

/// Creates a new role
pub async fn create(role: RoleCreate, db: &DbConn) -> Result<Role, DbErr> {
    let active_model: role::ActiveModel = role.into();
    active_model.insert(db).await.map(|model| model.into())
}

/// Updates a role
pub async fn update(role: RoleUpdate, db: &DbConn) -> Result<Role, DbErr> {
    let active_model: role::ActiveModel = role.into();
    active_model.insert(db).await.map(|model| model.into())
}

/// Deletes a role
pub async fn delete<F>(id: Option<i32>, condition: Option<F>, db: &DbConn) -> Result<u64, DbErr>
    where
        F: IntoCondition,
{
    let mut query = if let Some(id) = id {
        role::Entity::delete_many().filter(role::Column::Id.eq(id))
    } else {
        role::Entity::delete_many()
    };

    if let Some(condition) = condition {
        query = query.filter(condition);
    }

    query.exec(db).await.map(|res| res.rows_affected)
}
