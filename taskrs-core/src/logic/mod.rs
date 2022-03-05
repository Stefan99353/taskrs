pub mod auth;
mod permission;
mod refresh_token;
mod role;
mod role_permission;
mod user;
mod user_permission;
mod user_role;

use crate::models::IntoActiveModel;
use async_trait::async_trait;
use futures::try_join;
use taskrs_db::sea_orm::sea_query::SimpleExpr;
use taskrs_db::sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, Condition, ConnectionTrait, DbErr, DeleteResult,
    EntityTrait, InsertResult, ModelTrait, Order, PaginatorTrait, PrimaryKeyTrait, QueryFilter,
    QueryOrder,
};

#[async_trait]
pub trait CreateModelTrait<E, A, CM>
where
    E: EntityTrait,
    <E as EntityTrait>::Model: taskrs_db::sea_orm::IntoActiveModel<A> + Sync,
    A: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Send + 'static,
    CM: IntoActiveModel<A> + Send + 'static,
    Self: From<<E as EntityTrait>::Model> + Sized,
{
    /// Create a new entity
    async fn create<'a, C>(model: CM, db: &'a C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        model.into_active_model().insert(db).await.map(Self::from)
    }

    /// Create multiple new entities
    async fn create_many<'a, C>(models: Vec<CM>, db: &'a C) -> Result<InsertResult<A>, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        let active_models = models.into_iter().map(|m| m.into_active_model());

        E::insert_many(active_models).exec(db).await
    }
}

#[async_trait]
pub trait ReadModelTrait<E>
where
    E: EntityTrait,
    Self: From<<E as EntityTrait>::Model> + Sized,
{
    /// Get all entities
    async fn all<'a, C>(db: &'a C) -> Result<Vec<Self>, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        E::find()
            .all(db)
            .await
            .map(|res| res.into_iter().map(Self::from).collect())
    }

    /// Get a single entity by its id
    async fn get<'a, C>(
        id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
        db: &'a C,
    ) -> Result<Option<Self>, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        E::find_by_id(id)
            .one(db)
            .await
            .map(|res| res.map(Self::from))
    }

    /// Find multiple entities using a condition
    async fn find<'a, C>(condition: Condition, db: &'a C) -> Result<Vec<Self>, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        E::find()
            .filter(condition)
            .all(db)
            .await
            .map(|res| res.into_iter().map(Self::from).collect())
    }

    /// Find single entity using a condition
    async fn find_one<'a, C>(condition: Condition, db: &'a C) -> Result<Option<Self>, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        E::find()
            .filter(condition)
            .one(db)
            .await
            .map(|res| res.map(Self::from))
    }
}

#[async_trait]
pub trait UpdateModelTrait<E, A, UM>
where
    E: EntityTrait,
    <E as EntityTrait>::Model: taskrs_db::sea_orm::IntoActiveModel<A> + Sync,
    A: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Send + 'static,
    UM: IntoActiveModel<A> + Send + 'static,
    Self: From<<E as EntityTrait>::Model> + Sized,
{
    /// Update an entity
    async fn update<'a, C>(model: UM, db: &'a C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        model.into_active_model().update(db).await.map(Self::from)
    }
}

#[async_trait]
pub trait DeleteModelTrait<E, A>
where
    E: EntityTrait,
    <E as EntityTrait>::Model: taskrs_db::sea_orm::IntoActiveModel<A> + Sync,
    A: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Send + 'static,
    Self: From<<E as EntityTrait>::Model> + Sized,
{
    /// Delete entity using its id
    async fn delete<'a, C>(
        id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
        db: &'a C,
    ) -> Result<(), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        let model = E::find_by_id(id).one(db).await?;

        if let Some(model) = model {
            model.delete(db).await?;
        }

        Ok(())
    }

    /// Delete entities using a condition
    async fn delete_condition<'a, C>(condition: Condition, db: &'a C) -> Result<DeleteResult, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        E::delete_many().filter(condition).exec(db).await
    }
}

#[async_trait]
pub trait PaginatedModelTrait<E, A>
where
    E: EntityTrait,
    <E as EntityTrait>::Model: taskrs_db::sea_orm::IntoActiveModel<A> + Sync,
    A: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Send + 'static,
    Self: From<<E as EntityTrait>::Model> + Sized,
{
    /// Default order used in paginated requests
    fn default_order() -> (SimpleExpr, Order);

    /// Get all entities paginated
    async fn all_paginated<'a, C>(
        page: usize,
        limit: usize,
        order: Option<Vec<(SimpleExpr, Order)>>,
        db: &'a C,
    ) -> Result<(Vec<Self>, usize), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        let mut query = E::find();

        let mut ordered = false;
        if let Some(order) = order {
            ordered = !order.is_empty();
            for (col, ord) in order {
                query = query.order_by(col, ord);
            }
        }

        if !ordered {
            let (col, ord) = Self::default_order();
            query = query.order_by(col, ord);
        }

        let paginator = PaginatorTrait::paginate(query, db, limit);

        let (models, count) = try_join!(paginator.fetch_page(page), paginator.num_items())?;

        Ok((models.into_iter().map(Self::from).collect(), count))
    }

    /// Find entities paginated
    async fn find_paginated<'a, C>(
        condition: Condition,
        page: usize,
        limit: usize,
        order: Option<Vec<(SimpleExpr, Order)>>,
        db: &'a C,
    ) -> Result<(Vec<Self>, usize), DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        let mut query = E::find().filter(condition);

        let mut ordered = false;
        if let Some(order) = order {
            ordered = !order.is_empty();
            for (col, ord) in order {
                query = query.order_by(col, ord);
            }
        }

        if !ordered {
            let (col, ord) = Self::default_order();
            query = query.order_by(col, ord);
        }

        let paginator = PaginatorTrait::paginate(query, db, limit);

        let (models, count) = try_join!(paginator.fetch_page(page), paginator.num_items())?;

        Ok((models.into_iter().map(Self::from).collect(), count))
    }
}
