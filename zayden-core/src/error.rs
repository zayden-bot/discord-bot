#[derive(Debug)]
pub enum Error {
    MissingGuildId,
    NotInteractionAuthor,

    MessageConflict,
    //region: Serenity
    UnknownInteraction,
    ChannelDeleted,
    //endregion
    //region: Sqlx
    PoolTimedOut,
    //endregion
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MissingGuildId => write!(f, "This command can only be used within a server."),
            Error::NotInteractionAuthor => write!(f, "You are not the author of this interaction."),
            Error::MessageConflict => write!(
                f,
                "Command is already awaiting interaction. Please respond to previous command first."
            ),
            Error::UnknownInteraction => write!(
                f,
                "An error occurred while processing the interaction. Please try again."
            ),
            Error::ChannelDeleted => write!(f, "Channel already deleted"),
            Error::PoolTimedOut => write!(
                f,
                "An internal error occurred while accessing data. Please try again shortly."
            ),
        }
    }
}
