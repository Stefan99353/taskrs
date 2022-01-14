use crate::models::refresh_token;
use crate::models::refresh_token::dtos::{RefreshToken, RefreshTokenCreate, RefreshTokenUpdate};
use crate::sea_query::IntoCondition;
use futures::try_join;
use sea_orm::prelude::*;
use sea_orm::{DbConn, DbErr, IntoSimpleExpr, Order, QueryOrder};

/// Gets all refresh tokens from database
pub async fn get_all<F, C>(
    condition: Option<F>,
    order: Option<Vec<(Order, C)>>,
    db: &DbConn,
) -> Result<Vec<RefreshToken>, DbErr>
where
    F: IntoCondition,
    C: IntoSimpleExpr,
{
    let mut query = refresh_token::Entity::find();

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

/// Gets all refresh tokens from database in a paginated form
pub async fn get_paginated<F, C>(
    page: usize,
    limit: usize,
    condition: Option<F>,
    order: Option<Vec<(Order, C)>>,
    db: &DbConn,
) -> Result<(Vec<RefreshToken>, usize), DbErr>
where
    F: IntoCondition,
    C: IntoSimpleExpr,
{
    let mut query = refresh_token::Entity::find();

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

/// Gets a single refresh token from the database using an ID and/or a condition
pub async fn get<F>(
    id: Option<i32>,
    condition: Option<F>,
    db: &DbConn,
) -> Result<Option<RefreshToken>, DbErr>
where
    F: IntoCondition,
{
    let mut query = if let Some(id) = id {
        refresh_token::Entity::find_by_id(id)
    } else {
        refresh_token::Entity::find()
    };

    if let Some(condition) = condition {
        query = query.filter(condition);
    }

    query.one(db).await.map(|opt| opt.map(|model| model.into()))
}

/// Creates a new refresh token
pub async fn create(refresh_token: RefreshTokenCreate, db: &DbConn) -> Result<RefreshToken, DbErr> {
    let active_model: refresh_token::ActiveModel = refresh_token.into();
    active_model.insert(db).await.map(|model| model.into())
}

/// Updates a refresh token
pub async fn update(refresh_token: RefreshTokenUpdate, db: &DbConn) -> Result<RefreshToken, DbErr> {
    let active_model: refresh_token::ActiveModel = refresh_token.into();
    active_model.insert(db).await.map(|model| model.into())
}

/// Deletes a refresh token
pub async fn delete<F>(id: Option<i32>, condition: Option<F>, db: &DbConn) -> Result<u64, DbErr>
where
    F: IntoCondition,
{
    let mut query = if let Some(id) = id {
        refresh_token::Entity::delete_many().filter(refresh_token::Column::Id.eq(id))
    } else {
        refresh_token::Entity::delete_many()
    };

    if let Some(condition) = condition {
        query = query.filter(condition);
    }

    query.exec(db).await.map(|res| res.rows_affected)
}
