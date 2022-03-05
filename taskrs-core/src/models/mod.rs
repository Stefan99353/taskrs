pub mod auth;
pub mod permission;
pub mod refresh_token;
pub mod role;
pub mod role_permission;
pub mod user;
pub mod user_permission;
pub mod user_role;

use taskrs_db::sea_orm::ActiveModelTrait;

// Define trait in this to work around compiler error
pub trait IntoActiveModel<A>
where
    A: ActiveModelTrait,
{
    /// Method to call to perform the conversion
    fn into_active_model(self) -> A;
}
