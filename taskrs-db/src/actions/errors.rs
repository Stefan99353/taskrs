use std::fmt::{Formatter};

/// Error typed returned from actions that create/alter a user.
///
/// Holds two error types: [argon2::Error] or [sea_orm::DbErr] and custom errors
#[derive(Debug)]
pub enum AlterUserError {
    /// Error from password hashing
    Argon(argon2::Error),
    /// Error from db operation
    Db(sea_orm::DbErr),
    /// Email is already in use
    EmailExists,
}

impl std::error::Error for AlterUserError {}

impl std::fmt::Display for AlterUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AlterUserError::Argon(err) => write!(f, "{}", err),
            AlterUserError::Db(err) => write!(f, "{}", err),
            AlterUserError::EmailExists => write!(f, "New user email is already in use"),
        }
    }
}
