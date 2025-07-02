use serenity::all::DiscordJsonError;
use serenity::all::HttpError;
use zayden_core::Error as ZaydenError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingGuildId,
    NotInSupportChannel,

    Serenity(serenity::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MissingGuildId => ZaydenError::MissingGuildId.fmt(f),
            Error::NotInSupportChannel => {
                write!(f, "This command only works in the support channel.")
            }

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
                    error: DiscordJsonError { code: 50083, .. },
                    ..
                },
            ))) => write!(f, "This thread has already been closed and archived."),
            Self::Serenity(e) => unimplemented!("Unhandled serenity error: {e:?}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<serenity::Error> for Error {
    fn from(value: serenity::Error) -> Self {
        Self::Serenity(value)
    }
}
