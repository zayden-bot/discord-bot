use zayden_core::Error as ZaydenError;
use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnknownInteraction,
    MissingGuildId,
    PatreonAccountNotFound(String),
    NotInteractionAuthor,
    NegativeHours,
    CommandTimeout,
    PatreonTierTooLow,

    EndgameAnalysis(endgame_analysis::Error),
    Lfg(lfg::Error),
    ReactionRole(reaction_roles::Error),
    Ticket(ticket::Error),
    Suggestions(suggestions::Error),
    TempVoice(temp_voice::Error),

    Sqlx(sqlx::Error),
}

impl ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Error::UnknownInteraction => ZaydenError::UnknownInteraction.to_response(),
            Error::MissingGuildId => ZaydenError::MissingGuildId.to_response(),
            Error::PatreonAccountNotFound(_) => {
                "Patreon account not found.\nIf you've recently joined, please use `/patreon_user login` to manually update the cache and link your Discord account."
            }
            Error::NotInteractionAuthor => "You are not the author of this interaction.",
            Error::NegativeHours => "Hours must be a positive number.",
            Error::CommandTimeout => "You have already used this command today.",
            Error::PatreonTierTooLow => {
                "To access College Kings 2, you need to be an active $10 (Junior) patron with a lifetime subscription of $20.\nUse `/patreon_user login` to manually update the cache and link your Discord account."
            }

            Error::EndgameAnalysis(e) => e.to_response(),
            Error::Lfg(e) => e.to_response(),
            Error::ReactionRole(e) => e.to_response(),
            Error::Ticket(e) => e.to_response(),
            Error::Suggestions(e) => e.to_response(),
            Error::TempVoice(e) => e.to_response(),

            Error::Sqlx(_) => {
                "An error occurred while processing your request. Please try again later."
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<endgame_analysis::Error> for Error {
    fn from(e: endgame_analysis::Error) -> Self {
        Error::EndgameAnalysis(e)
    }
}

impl From<lfg::Error> for Error {
    fn from(e: lfg::Error) -> Self {
        Error::Lfg(e)
    }
}

impl From<reaction_roles::Error> for Error {
    fn from(e: reaction_roles::Error) -> Self {
        Error::ReactionRole(e)
    }
}

impl From<temp_voice::Error> for Error {
    fn from(e: temp_voice::Error) -> Self {
        Error::TempVoice(e)
    }
}

impl From<ticket::Error> for Error {
    fn from(e: ticket::Error) -> Self {
        Error::Ticket(e)
    }
}

impl From<suggestions::Error> for Error {
    fn from(e: suggestions::Error) -> Self {
        Error::Suggestions(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::PoolTimedOut => Error::Sqlx(e),
            _ => panic!("Unhandled SQLx error: {:?}", e),
        }
    }
}
