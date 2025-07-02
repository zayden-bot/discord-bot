use serenity::all::{DiscordJsonError, ErrorResponse, HttpError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    WeaponNotFound(String),

    UnknownInteraction,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::WeaponNotFound(weapon) => write!(f, "Weapon {} not found", weapon),
            Self::UnknownInteraction => write!(
                f,
                "An error occurred while processing the interaction. Please try again."
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<serenity::Error> for Error {
    fn from(value: serenity::Error) -> Self {
        match value {
            serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 10062, .. },
                ..
            })) => Self::UnknownInteraction,
            e => unimplemented!("Unhandled Serenity error: {e:?}"),
        }
    }
}
