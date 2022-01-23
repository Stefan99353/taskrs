use crate::actions::errors::AlterUserError;
use crate::models::user;
use crate::models::user::dtos::{User, UserCreate, UserUpdate};
use futures::try_join;
use sea_orm::prelude::*;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{Condition, Order, QueryOrder};

/// Gets all users from database
pub async fn get_all(
    condition: Option<Condition>,
    order: Option<Vec<(Order, SimpleExpr)>>,
    db: &DbConn,
) -> Result<Vec<User>, DbErr> {
    let mut query = user::Entity::find();

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

/// Gets all users from database in a paginated form
pub async fn get_paginated(
    page: usize,
    limit: usize,
    condition: Option<Condition>,
    order: Option<Vec<(Order, SimpleExpr)>>,
    db: &DbConn,
) -> Result<(Vec<User>, usize), DbErr> {
    let mut query = user::Entity::find();

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

/// Gets a single user from the database using an ID and/or a condition
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
        query = query.filter(condition);
    }

    query.one(db).await.map(|opt| opt.map(|model| model.into()))
}

/// Check if a email is already in use
pub async fn check_email_exists(email: &str, db: &DbConn) -> Result<bool, DbErr> {
    user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
        .map(|user| user.is_some())
}

/// Creates a new user and checks if the email is unique
pub async fn create(user: UserCreate, db: &DbConn) -> Result<User, AlterUserError> {
    let exists = check_email_exists(&user.email, db)
        .await
        .map_err(AlterUserError::Db)?;
    if exists {
        return Err(AlterUserError::EmailExists);
    }

    let active_model: user::ActiveModel = user.try_into().map_err(AlterUserError::Argon)?;
    active_model
        .insert(db)
        .await
        .map(|model| model.into())
        .map_err(AlterUserError::Db)
}

/// Updates a user and checks if the email is unique
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
    active_model
        .update(db)
        .await
        .map(|model| model.into())
        .map_err(AlterUserError::Db)
}

/// Deletes a user
pub async fn delete(
    id: Option<i32>,
    condition: Option<Condition>,
    db: &DbConn,
) -> Result<u64, DbErr> {
    let mut query = if let Some(id) = id {
        user::Entity::delete_many().filter(user::Column::Id.eq(id))
    } else {
        user::Entity::delete_many()
    };

    if let Some(condition) = condition {
        query = query.filter(condition);
    }

    query.exec(db).await.map(|res| res.rows_affected)
}
