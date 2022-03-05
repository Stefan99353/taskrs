#[derive(Debug)]
pub enum Error {
    Argon(argon2::Error),
    Auth(AuthError),
    Database(taskrs_db::sea_orm::DbErr),
    JsonWebToken(jsonwebtoken::errors::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Argon(e) => write!(f, "Password hashing/verifying error: {}", e),
            Self::Auth(e) => write!(f, "Auth Error: {}", e),
            Self::Database(e) => write!(f, "Database Error: {}", e),
            Self::JsonWebToken(e) => write!(f, "Error while creating/decoding JWTs: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    UnknownEmail,
    UserDisabled,
    WrongPassword,
}

impl std::error::Error for AuthError {}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownEmail => write!(f, "The provided email is not known"),
            Self::UserDisabled => write!(f, "The requested user is disabled"),
            Self::WrongPassword => write!(f, "The provided password is wrong"),
        }
    }
}
