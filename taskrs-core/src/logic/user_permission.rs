use crate::logic::{CreateModelTrait, DeleteModelTrait, ReadModelTrait};
use crate::models::user_permission::UserPermission;
use taskrs_db::models::user_permission;

impl CreateModelTrait<user_permission::Entity, user_permission::ActiveModel, UserPermission>
    for UserPermission
{
}

impl ReadModelTrait<user_permission::Entity> for UserPermission {}

impl DeleteModelTrait<user_permission::Entity, user_permission::ActiveModel> for UserPermission {}
