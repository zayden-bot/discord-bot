use std::fmt;
use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
pub enum Error {
    Dotenvy(dotenvy::Error),
    Serenity(serenity::Error),
    Sqlx(sqlx::Error),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Dotenvy(e) => write!(f, "Dotenvy error: {}", e),
            Error::Serenity(e) => write!(f, "Serenity error: {}", e),
            Error::Sqlx(e) => write!(f, "Sqlx error: {}", e),
            Error::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl From<dotenvy::Error> for Error {
    fn from(e: dotenvy::Error) -> Self {
        Error::Dotenvy(e)
    }
}

impl From<serenity::Error> for Error {
    fn from(e: serenity::Error) -> Self {
        Error::Serenity(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Sqlx(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Other(e)
    }
}
