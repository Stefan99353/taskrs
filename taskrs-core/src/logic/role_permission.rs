use crate::logic::{CreateModelTrait, DeleteModelTrait, ReadModelTrait};
use crate::models::role_permission::RolePermission;
use taskrs_db::models::role_permission;

impl CreateModelTrait<role_permission::Entity, role_permission::ActiveModel, RolePermission>
    for RolePermission
{
}

impl ReadModelTrait<role_permission::Entity> for RolePermission {}

impl DeleteModelTrait<role_permission::Entity, role_permission::ActiveModel> for RolePermission {}
