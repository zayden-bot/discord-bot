use serenity::all::{DiscordJsonError, HttpError, StatusCode};
use zayden_core::Error as ZaydenError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingGuildId,
    NotInteractionAuthor,
    NegativeHours,

    EndgameAnalysis(endgame_analysis::Error),
    Gambling(gambling::Error),
    Lfg(lfg::Error),
    ReactionRole(reaction_roles::Error),
    Ticket(ticket::Error),
    Suggestions(suggestions::Error),
    TempVoice(temp_voice::Error),

    Serenity(serenity::Error),
    Sqlx(sqlx::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MissingGuildId => ZaydenError::MissingGuildId.fmt(f),
            Error::NotInteractionAuthor => write!(f, "You are not the author of this interaction."),
            Error::NegativeHours => write!(f, "Hours must be a positive number."),

            Error::EndgameAnalysis(e) => e.fmt(f),
            Error::Gambling(e) => e.fmt(f),
            Error::Lfg(e) => e.fmt(f),
            Error::ReactionRole(e) => e.fmt(f),
            Error::Ticket(e) => e.fmt(f),
            Error::Suggestions(e) => e.fmt(f),
            Error::TempVoice(e) => e.fmt(f),

            Self::Serenity(serenity::Error::Http(HttpError::UnsuccessfulRequest(
                serenity::all::ErrorResponse {
                    error: DiscordJsonError { code: 10003, .. },
                    ..
                },
            ))) => zayden_core::Error::ChannelDeleted.fmt(f),

            Self::Serenity(serenity::Error::Http(HttpError::UnsuccessfulRequest(
                serenity::all::ErrorResponse {
                    error: DiscordJsonError { code: 10008, .. },
                    ..
                },
            ))) => write!(f, "Message was unexpectably deleted. Please try again."),
            Self::Serenity(serenity::Error::Http(HttpError::UnsuccessfulRequest(
                serenity::all::ErrorResponse {
                    error: DiscordJsonError { code: 10062, .. },
                    ..
                },
            ))) => ZaydenError::UnknownInteraction.fmt(f),
            Self::Serenity(serenity::Error::Http(HttpError::UnsuccessfulRequest(
                serenity::all::ErrorResponse {
                    error: DiscordJsonError { code: 50001, .. },
                    ..
                },
            ))) => write!(
                f,
                "I'm missing access perform that action. Please contact a server admin to resolve this."
            ),
            Self::Serenity(serenity::Error::Http(HttpError::UnsuccessfulRequest(
                serenity::all::ErrorResponse {
                    error: DiscordJsonError { code: 50013, .. },
                    ..
                },
            ))) => {
                write!(
                    f,
                    "I'm missing permissions perform that action. Please contact a server admin to resolve this."
                )
            }
            Self::Serenity(serenity::Error::Http(HttpError::UnsuccessfulRequest(
                serenity::all::ErrorResponse {
                    error: DiscordJsonError { code: 50083, .. },
                    ..
                },
            ))) => write!(f, "This thread has already been closed and archived."),
            Error::Serenity(serenity::Error::Http(HttpError::UnsuccessfulRequest(
                serenity::all::ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR | StatusCode::SERVICE_UNAVAILABLE,
                    ..
                },
            ))) => write!(
                f,
                "It looks like Discord is currently experiencing some server issues. Please try your request again shortly. If the problem persists, please contact OscarSix."
            ),
            Error::Serenity(e) => unimplemented!("Unhandled Serenity error: {e:?}"),

            Error::Sqlx(sqlx::Error::PoolTimedOut) => ZaydenError::PoolTimedOut.fmt(f),
            Error::Sqlx(sqlx::Error::ColumnDecode { source, .. })
                if source.is::<sqlx::error::UnexpectedNullError>() =>
            {
                write!(
                    f,
                    "There is a problem processing your request. Please try again. If the issue persists, please contact OscarSix."
                )
            }
            Error::Sqlx(e) => unimplemented!("Unhandled SQLx error: {e:?}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<endgame_analysis::Error> for Error {
    fn from(e: endgame_analysis::Error) -> Self {
        Error::EndgameAnalysis(e)
    }
}

impl From<gambling::Error> for Error {
    fn from(value: gambling::Error) -> Self {
        match value {
            gambling::Error::Serenity(e) => Self::Serenity(e),
            gambling::Error::Sqlx(e) => Self::Sqlx(e),
            value => Self::Gambling(value),
        }
    }
}

impl From<lfg::Error> for Error {
    fn from(value: lfg::Error) -> Self {
        match value {
            lfg::Error::Serenity(e) => Self::Serenity(e),
            value => Self::Lfg(value),
        }
    }
}

impl From<reaction_roles::Error> for Error {
    fn from(e: reaction_roles::Error) -> Self {
        Error::ReactionRole(e)
    }
}

impl From<temp_voice::Error> for Error {
    fn from(value: temp_voice::Error) -> Self {
        match value {
            temp_voice::Error::Serenity(e) => Self::Serenity(e),
            value => Error::TempVoice(value),
        }
    }
}

impl From<ticket::Error> for Error {
    fn from(value: ticket::Error) -> Self {
        match value {
            ticket::Error::MissingGuildId => Self::MissingGuildId,
            ticket::Error::Serenity(e) => Self::Serenity(e),
            value => Self::Ticket(value),
        }
    }
}

impl From<suggestions::Error> for Error {
    fn from(value: suggestions::Error) -> Self {
        match value {
            suggestions::Error::MissingGuildId => Self::MissingGuildId,
            value => Self::Suggestions(value),
        }
    }
}

impl From<serenity::Error> for Error {
    fn from(value: serenity::Error) -> Self {
        Self::Serenity(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}
