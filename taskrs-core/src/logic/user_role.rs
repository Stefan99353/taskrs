use crate::logic::{CreateModelTrait, DeleteModelTrait, ReadModelTrait};
use crate::models::user_role::UserRole;
use taskrs_db::models::user_role;

impl CreateModelTrait<user_role::Entity, user_role::ActiveModel, UserRole> for UserRole {}

impl ReadModelTrait<user_role::Entity> for UserRole {}

impl DeleteModelTrait<user_role::Entity, user_role::ActiveModel> for UserRole {}
