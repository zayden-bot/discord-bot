use serenity::all::{CommandInteraction, Context, Mentionable};

use crate::{utils::message_response, OSCAR_SIX_ID};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConversionError,
    CommandNotFound(String),
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
    NoSupportThread,
    NoSpoilerThread,
    FaqMessageNotFound(String),
    EmptyMessage,
    PatreonAccountNotFound(String),
    NotInGuild,

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
}

impl Error {
    pub async fn to_response(self, ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let msg = match self {
            Error::PatreonAccountNotFound(_) => String::from("Patreon account not found.\nIf you've recently joined, please use `/patreon login` to manually update the cache and link your Discord account."),
            _ => String::new(),
        };

        if msg.is_empty() {
            message_response(
                ctx,
                interaction,
                format!(
                    "An error occurred. Please contact {} if this issue persists.",
                    OSCAR_SIX_ID.mention()
                ),
            )
            .await?;

            Err(self)
        } else {
            message_response(ctx, interaction, msg).await?;
            Ok(())
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

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
