use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
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

    Family(family::Error),
    GoldStar(gold_star::Error),
    ReactionRole(reaction_roles::Error),
    Ticket(ticket::Error),
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
