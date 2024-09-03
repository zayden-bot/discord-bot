use serenity::all::{ChannelId, CreateCommand, GuildId};

use crate::guild_commands::college_kings::*;

pub const GUILD_ID: GuildId = GuildId::new(745662812335898806);

pub const SUPPORT_FAQ_CHANNEL_ID: ChannelId = ChannelId::new(1224557981849485392);

pub const INFORMATION_CHANNEL_ID: ChannelId = ChannelId::new(830927865784565800);
pub const CHANGE_LOG_CHANNEL_ID: ChannelId = ChannelId::new(992599169288122410);
pub const RENDER_REQUESTS_CHANNEL_ID: ChannelId = ChannelId::new(1235269134246346816);
pub const SUGGESTION_CATEGORY_ID: ChannelId = ChannelId::new(1068790374996377671);
pub const FAQ_CHANNEL_ID: ChannelId = ChannelId::new(1196346920059289690);

pub fn commands() -> Vec<CreateCommand> {
    vec![
        add_artist::register(),
        availability_check::register(),
        close::register(),
        faq::register(),
        fetch_suggestions::register(),
        fixed::register(),
        get_discord_role::register(),
        goodmorning::register(),
        goodnight::register(),
        image::register(),
        open::register(),
        reputation::register(),
        saves::register(),
        spoilers::register(),
        support::register(),
    ]
}
