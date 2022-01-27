use crate::actions::errors::AlterUserError;
use crate::models::user;
use crate::models::user::dtos::{User, UserCreate, UserUpdate};
use futures::try_join;
use sea_orm::prelude::*;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{Condition, ConnectionTrait, Order, QueryOrder};

/// Gets all users from database
#[instrument(
    name = "get_all_users"
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
) -> Result<Vec<User>, DbErr>
where
    C: ConnectionTrait<'a>,
{
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
) -> Result<(Vec<User>, usize), DbErr>
where
    C: ConnectionTrait<'a>,
{
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
    fields (
        id = tracing::field::debug(id),
        condition = tracing::field::debug(&condition),
    )
)]
pub async fn get<'a, C>(
    id: Option<i32>,
    condition: Option<Condition>,
    db: &'a C,
) -> Result<Option<User>, DbErr>
where
    C: ConnectionTrait<'a>,
{
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
    fields (
        email = email
    )
)]
pub async fn check_email_exists<'a, C>(email: &str, db: &'a C) -> Result<bool, DbErr>
where
    C: ConnectionTrait<'a>,
{
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
)]
pub async fn create<'a, C>(user: UserCreate, db: &'a C) -> Result<User, AlterUserError>
where
    C: ConnectionTrait<'a>,
{
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

/// Create new users and returns last inserted id. Does not check if emails exist.
#[instrument(
name = "create_users"
level = "debug",
skip_all,
)]
pub async fn create_many<'a, C>(users: Vec<UserCreate>, db: &'a C) -> Result<i32, AlterUserError>
where
    C: ConnectionTrait<'a>,
{
    let active_models: Vec<user::ActiveModel> = users
        .into_iter()
        .map(|p| p.try_into().map_err(AlterUserError::Argon))
        .collect::<Result<_, AlterUserError>>()?;

    debug!("Inserting new users");
    user::Entity::insert_many(active_models)
        .exec(db)
        .await
        .map(|r| r.last_insert_id)
        .map_err(AlterUserError::Db)
}

/// Updates a user and checks if the email is unique
#[instrument(
    name = "update_user"
    level = "debug",
    skip_all,
    fields (
        id = user.id
    )
)]
pub async fn update<'a, C>(user: UserUpdate, db: &'a C) -> Result<User, AlterUserError>
where
    C: ConnectionTrait<'a>,
{
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
