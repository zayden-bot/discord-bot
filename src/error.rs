use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConversionError,
    UnknownCommand(String),
    CommandNotFound,
    DataNotFound,
    TimeDelta,
    NoImage,
    NoUser,
    UserNotFound,
    NoRole,
    RoleNotFound(u64),
    NoMember,
    NoChannel,
    NoParent,
    NoFileName,
    FaqMessageNotFound(String),
    EmptyMessage,
    PatreonAccountNotFound(String),
    NotInGuild,
    NotInteractionAuthor,

    ServersTable(crate::guilds::ServersTableError),

    Family(family::Error),
    GoldStar(gold_star::Error),
    ReactionRole(reaction_roles::Error),
    Ticket(ticket::Error),

    Dotenvy(dotenvy::Error),
    Serenity(serenity::Error),
    SerenityTimestamp(serenity::model::timestamp::InvalidTimestamp),
    Sqlx(sqlx::Error),
    EnvVar(std::env::VarError),
    Reqwest(reqwest::Error),
    Cron(cron::error::Error),
    ParseIntError(std::num::ParseIntError),
    ReactionConversionError(serenity::all::ReactionConversionError),
    JoinError(tokio::task::JoinError),
    Bunny(bunny_cdn_wrapper::Error),
    // Charming(charming::EchartsError),
    Io(std::io::Error),
    TryFromInt(std::num::TryFromIntError),
}

impl ErrorResponse for Error {
    fn to_response(&self) -> String {
        match self {
            Error::PatreonAccountNotFound(_) => String::from("Patreon account not found.\nIf you've recently joined, please use `/patreon_user login` to manually update the cache and link your Discord account."),
            Error::NotInteractionAuthor => String::from("You are not the author of this interaction."),
            Error::Family(e) => e.to_response(),
            Error::GoldStar(e) => e.to_response(),
            Error::ReactionRole(e) => e.to_response(),
            Error::Ticket(e) => e.to_response(),
            _ => String::new(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<crate::guilds::ServersTableError> for Error {
    fn from(e: crate::guilds::ServersTableError) -> Self {
        Error::ServersTable(e)
    }
}

impl From<family::Error> for Error {
    fn from(e: family::Error) -> Self {
        Error::Family(e)
    }
}

impl From<gold_star::Error> for Error {
    fn from(e: gold_star::Error) -> Self {
        Error::GoldStar(e)
    }
}

impl From<reaction_roles::Error> for Error {
    fn from(e: reaction_roles::Error) -> Self {
        Error::ReactionRole(e)
    }
}

impl From<ticket::Error> for Error {
    fn from(e: ticket::Error) -> Self {
        Error::Ticket(e)
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

impl From<serenity::all::ReactionConversionError> for Error {
    fn from(e: serenity::all::ReactionConversionError) -> Self {
        Error::ReactionConversionError(e)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Error::JoinError(e)
    }
}

impl From<bunny_cdn_wrapper::Error> for Error {
    fn from(e: bunny_cdn_wrapper::Error) -> Self {
        Error::Bunny(e)
    }
}

// impl From<charming::EchartsError> for Error {
//     fn from(e: charming::EchartsError) -> Self {
//         Error::Charming(e)
//     }
// }

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(e: std::num::TryFromIntError) -> Self {
        Error::TryFromInt(e)
    }
}
