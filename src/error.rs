pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Dotenvy(dotenvy::Error),
    Serenity(serenity::Error),
    SerenityTimestamp(serenity::model::timestamp::InvalidTimestamp),
    Sqlx(sqlx::Error),
    EnvVar(std::env::VarError),
    Reqwest(reqwest::Error),
    Cron(cron::error::Error),
    ParseIntError(std::num::ParseIntError),
    ChronoError,
    ConversionError,
    DataNotFound,
    TimeDelta,
    NoImage,
    NoUser,
    UserNotFound,
    NoGuild,
    NoRole,
    RoleNotFound(u64),
    NoMember,
    NoParent,
    NoFileName,
    NoSupportThread,
    NoSpoilerThread,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Dotenvy(e) => write!(f, "{}", e),
            Error::Serenity(e) => write!(f, "{}", e),
            Error::SerenityTimestamp(e) => write!(f, "{}", e),
            Error::Sqlx(e) => write!(f, "{}", e),
            Error::EnvVar(e) => write!(f, "{}", e),
            Error::Reqwest(e) => write!(f, "{}", e),
            Error::Cron(e) => write!(f, "{}", e),
            Error::ParseIntError(e) => write!(f, "{}", e),
            Error::ChronoError => write!(f, "Chrono error"),
            Error::ConversionError => write!(f, "Conversion error"),
            Error::DataNotFound => write!(f, "Data not found"),
            Error::TimeDelta => write!(f, "TimeDelta error"),
            Error::NoImage => write!(f, "No image found"),
            Error::NoUser => write!(f, "No user found"),
            Error::UserNotFound => write!(f, "User not found"),
            Error::NoGuild => write!(f, "No guild found"),
            Error::NoRole => write!(f, "No role found"),
            Error::RoleNotFound(id) => write!(f, "Role not found: {}", id),
            Error::NoMember => write!(f, "No member found"),
            Error::NoParent => write!(f, "No parent channel found"),
            Error::NoFileName => write!(f, "No file name found"),
            Error::NoSupportThread => write!(f, "No support thread found"),
            Error::NoSpoilerThread => write!(f, "No spoiler thread found"),
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

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::EnvVar(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<cron::error::Error> for Error {
    fn from(e: cron::error::Error) -> Self {
        Error::Cron(e)
    }
}

impl From<serenity::model::timestamp::InvalidTimestamp> for Error {
    fn from(e: serenity::model::timestamp::InvalidTimestamp) -> Self {
        Error::SerenityTimestamp(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseIntError(e)
    }
}
