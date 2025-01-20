use zayden_core::Error as ZaydenError;
use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingGuildId,
    PatreonAccountNotFound(String),
    NotInteractionAuthor,
    NegativeHours,
    CommandTimeout,
    PatreonTierTooLow,

    GoldStar(gold_star::Error),
    ReactionRole(reaction_roles::Error),
    Ticket(ticket::Error),
}

impl ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Error::MissingGuildId => ZaydenError::MissingGuildId.to_response(),
            Error::PatreonAccountNotFound(_) => "Patreon account not found.\nIf you've recently joined, please use `/patreon_user login` to manually update the cache and link your Discord account.",
            Error::NotInteractionAuthor => "You are not the author of this interaction.",
            Error::NegativeHours => "Hours must be a positive number.",
            Error::CommandTimeout => "You have already used this command today.",
            Error::PatreonTierTooLow => "To access College Kings 2, you need to be an active $10 (Junior) patron with a lifetime subscription of $20.\nUse `/patreon_user login` to manually update the cache and link your Discord account.",

            Error::GoldStar(e) => e.to_response(),
            Error::ReactionRole(e) => e.to_response(),
            Error::Ticket(e) => e.to_response(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

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
