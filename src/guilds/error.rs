#[derive(Debug)]
pub enum ServersTableError {
    ServerNotFound,
    RulesChannelNotFound,
    GeneralChannelNotFound,
    SpoilerChannelNotFound,
    SupportChannelNotFound,
    SuggestionsChannelNotFound,
    SupportRoleNotFound,
    ArtistRoleNotFound,
    SleepRoleNotFound,
}

impl std::fmt::Display for ServersTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ServersTableError {}
