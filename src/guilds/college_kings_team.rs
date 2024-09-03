use serenity::all::{ChannelId, CreateCommand, GuildId, RoleId, UserId};

use crate::guild_commands::college_kings_team::*;

pub const GUILD_ID: GuildId = GuildId::new(814314852855447634);

pub const TEAM_LEADS_CHANNEL_ID: ChannelId = ChannelId::new(846021706203136030);
pub const REVIEW_CHANNEL_ID: ChannelId = ChannelId::new(1227244734515380316);

pub const TEAM_LEADERS_ROLE_ID: RoleId = RoleId::new(836275726352646176);

pub const STEVE_USER_ID: UserId = UserId::new(747423760780623872);
pub const MESSY_USER_ID: UserId = UserId::new(841466088612298793);

pub fn commands() -> Vec<CreateCommand> {
    vec![review::register()]
}
