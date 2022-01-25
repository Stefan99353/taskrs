use crate::actions::errors::AlterUserError;
use crate::models::user;
use crate::models::user::dtos::{User, UserCreate, UserUpdate};
use futures::try_join;
use sea_orm::prelude::*;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{Condition, Order, QueryOrder};

/// Gets all users from database
#[instrument(
    name = "get_all_users"
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
) -> Result<Vec<User>, DbErr> {
    let mut query = user::Entity::find();

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

/// Gets all users from database in a paginated form
#[instrument(
    name = "get_paginated_users"
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
) -> Result<(Vec<User>, usize), DbErr> {
    let mut query = user::Entity::find();

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

/// Gets a single user from the database using an ID and/or a condition
#[instrument(
    name = "get_user"
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
) -> Result<Option<User>, DbErr> {
    let mut query = if let Some(id) = id {
        user::Entity::find_by_id(id)
    } else {
        user::Entity::find()
    };

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    debug!("Executing query");
    query.one(db).await.map(|opt| opt.map(|model| model.into()))
}

/// Check if a email is already in use
#[instrument(
    name = "check_user_email_exists"
    level = "debug",
    skip_all,
    err,
    fields (
        email = email
    )
)]
pub async fn check_email_exists(email: &str, db: &DbConn) -> Result<bool, DbErr> {
    debug!("Checking if user exists with email");
    user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
        .map(|user| user.is_some())
}

/// Creates a new user and checks if the email is unique
#[instrument(
    name = "create_user"
    level = "debug",
    skip_all,
    err,
)]
pub async fn create(user: UserCreate, db: &DbConn) -> Result<User, AlterUserError> {
    let exists = check_email_exists(&user.email, db)
        .await
        .map_err(AlterUserError::Db)?;
    if exists {
        return Err(AlterUserError::EmailExists);
    }

    let active_model: user::ActiveModel = user.try_into().map_err(AlterUserError::Argon)?;
    debug!("Inserting new user");
    active_model
        .insert(db)
        .await
        .map(|model| model.into())
        .map_err(AlterUserError::Db)
}

/// Updates a user and checks if the email is unique
#[instrument(
    name = "update_user"
    level = "debug",
    skip_all,
    err,
    fields (
        id = user.id
    )
)]
pub async fn update(user: UserUpdate, db: &DbConn) -> Result<User, AlterUserError> {
    if let Some(email) = &user.email {
        let exists = check_email_exists(email, db)
            .await
            .map_err(AlterUserError::Db)?;
        if exists {
            return Err(AlterUserError::EmailExists);
        }
    }

    let active_model: user::ActiveModel = user.try_into().map_err(AlterUserError::Argon)?;
    debug!("Updating permission");
    active_model
        .update(db)
        .await
        .map(|model| model.into())
        .map_err(AlterUserError::Db)
}

/// Deletes a user
#[instrument(
    name = "delete_users"
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
    let mut query = user::Entity::delete_many();

    if let Some(id) = id {
        trace!("Filter for ID");
        query = query.filter(user::Column::Id.eq(id))
    }

    if let Some(condition) = condition {
        trace!("Apply condition to query");
        query = query.filter(condition);
    }

    debug!("Executing query");
    query.exec(db).await.map(|res| res.rows_affected)
}
