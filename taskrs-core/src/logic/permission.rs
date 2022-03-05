use crate::logic::{
    CreateModelTrait, DeleteModelTrait, PaginatedModelTrait, ReadModelTrait, UpdateModelTrait,
};
use crate::models::permission::{Permission, PermissionCreate, PermissionUpdate};
use taskrs_db::models::permission;
use taskrs_db::sea_orm::sea_query::SimpleExpr;
use taskrs_db::sea_orm::{IntoSimpleExpr, Order};

impl CreateModelTrait<permission::Entity, permission::ActiveModel, PermissionCreate>
    for Permission
{
}

impl ReadModelTrait<permission::Entity> for Permission {}

impl UpdateModelTrait<permission::Entity, permission::ActiveModel, PermissionUpdate>
    for Permission
{
}

impl DeleteModelTrait<permission::Entity, permission::ActiveModel> for Permission {}

impl PaginatedModelTrait<permission::Entity, permission::ActiveModel> for Permission {
    fn default_order() -> (SimpleExpr, Order) {
        (permission::Column::Id.into_simple_expr(), Order::Asc)
    }
}
