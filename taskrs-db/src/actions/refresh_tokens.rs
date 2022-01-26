use crate::models::refresh_token;
use crate::models::refresh_token::dtos::{RefreshToken, RefreshTokenCreate, RefreshTokenUpdate};
use futures::try_join;
use sea_orm::prelude::*;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{Condition, DbConn, DbErr, Order, QueryOrder};

/// Gets all refresh tokens from database
#[instrument(
    name = "get_all_refresh_tokens"
    level = "debug",
    skip_all,
    fields (
        condition = tracing::field::debug(&condition),
        order = tracing::field::debug(&order),
    )
)]
pub async fn get_all(
    condition: Option<Condition>,
    order: Option<Vec<(Order, SimpleExpr)>>,
    db: &DbConn,
) -> Result<Vec<RefreshToken>, DbErr> {
    let mut query = refresh_token::Entity::find();

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

/// Gets all refresh tokens from database in a paginated form
#[instrument(
    name = "get_paginated_refresh_tokens"
    level = "debug",
    skip_all,
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
) -> Result<(Vec<RefreshToken>, usize), DbErr> {
    let mut query = refresh_token::Entity::find();

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

/// Gets a single refresh token from the database using an ID and/or a condition
#[instrument(
    name = "get_refresh_token"
    level = "debug",
    skip_all,
    fields (
        id = tracing::field::debug(id),
        condition = tracing::field::debug(&condition),
    )
)]
pub async fn get(
    id: Option<i32>,
    condition: Option<Condition>,
    db: &DbConn,
) -> Result<Option<RefreshToken>, DbErr> {
    let mut query = if let Some(id) = id {
        refresh_token::Entity::find_by_id(id)
    } else {
        refresh_token::Entity::find()
    };

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    debug!("Executing query");
    query.one(db).await.map(|opt| opt.map(|model| model.into()))
}

/// Creates a new refresh token
#[instrument(
    name = "create_refresh_token"
    level = "debug",
    skip_all,
)]
pub async fn create(refresh_token: RefreshTokenCreate, db: &DbConn) -> Result<RefreshToken, DbErr> {
    let active_model: refresh_token::ActiveModel = refresh_token.into();
    debug!("Inserting new permission");
    active_model.insert(db).await.map(|model| model.into())
}

/// Updates a refresh token
#[instrument(
    name = "update_refresh_token"
    level = "debug",
    skip_all,
    fields (
    id = refresh_token.id
    )
)]
pub async fn update(refresh_token: RefreshTokenUpdate, db: &DbConn) -> Result<RefreshToken, DbErr> {
    let active_model: refresh_token::ActiveModel = refresh_token.into();
    debug!("Updating permission");
    active_model.insert(db).await.map(|model| model.into())
}

/// Deletes a refresh token
#[instrument(
    name = "delete_refresh_tokens"
    level = "debug",
    skip_all,
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
    let mut query = refresh_token::Entity::delete_many();

    if let Some(id) = id {
        trace!("Filter for ID");
        query = query.filter(refresh_token::Column::Id.eq(id))
    }

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    debug!("Executing query");
    query.exec(db).await.map(|res| res.rows_affected)
}
