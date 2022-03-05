use crate::logic::{
    CreateModelTrait, DeleteModelTrait, PaginatedModelTrait, ReadModelTrait, UpdateModelTrait,
};
use crate::models::refresh_token::{RefreshToken, RefreshTokenCreate, RefreshTokenUpdate};
use taskrs_db::models::refresh_token;
use taskrs_db::sea_orm::sea_query::SimpleExpr;
use taskrs_db::sea_orm::{IntoSimpleExpr, Order};

impl CreateModelTrait<refresh_token::Entity, refresh_token::ActiveModel, RefreshTokenCreate>
    for RefreshToken
{
}

impl ReadModelTrait<refresh_token::Entity> for RefreshToken {}

impl UpdateModelTrait<refresh_token::Entity, refresh_token::ActiveModel, RefreshTokenUpdate>
    for RefreshToken
{
}

impl DeleteModelTrait<refresh_token::Entity, refresh_token::ActiveModel> for RefreshToken {}

impl PaginatedModelTrait<refresh_token::Entity, refresh_token::ActiveModel> for RefreshToken {
    fn default_order() -> (SimpleExpr, Order) {
        (refresh_token::Column::Id.into_simple_expr(), Order::Asc)
    }
}
