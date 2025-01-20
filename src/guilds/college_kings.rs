use serenity::all::{ChannelId, Context, CreateCommand, GuildId, Ready};
use zayden_core::SlashCommand;

use crate::guild_commands::college_kings::{
    AddArtist, AvailabilityCheck, Faq, FetchSuggestions, GetDiscordRole, Greetings, Reputation,
    Saves, Spoilers,
};
use crate::Result;

pub const GUILD_ID: GuildId = GuildId::new(745662812335898806);

pub const SUPPORT_FAQ_CHANNEL_ID: ChannelId = ChannelId::new(1224557981849485392);

pub const INFORMATION_CHANNEL_ID: ChannelId = ChannelId::new(830927865784565800);
pub const CHANGE_LOG_CHANNEL_ID: ChannelId = ChannelId::new(992599169288122410);
pub const RENDER_REQUESTS_CHANNEL_ID: ChannelId = ChannelId::new(1235269134246346816);
pub const SUGGESTION_CATEGORY_ID: ChannelId = ChannelId::new(1068790374996377671);
pub const FAQ_CHANNEL_ID: ChannelId = ChannelId::new(1196346920059289690);

pub fn commands(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![
        AddArtist::register(ctx, ready)?,
        AvailabilityCheck::register(ctx, ready)?,
        Faq::register(ctx, ready)?,
        FetchSuggestions::register(ctx, ready)?,
        GetDiscordRole::register(ctx, ready)?,
        Greetings::register(ctx, ready)?,
        Reputation::register(ctx, ready)?,
        Saves::register(ctx, ready)?,
        Spoilers::register(ctx, ready)?,
    ];

    Ok(commands)
}
