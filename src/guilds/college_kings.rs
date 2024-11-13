use serenity::all::{ChannelId, Context, CreateCommand, GuildId, Ready};

use crate::guild_commands::college_kings::*;
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
        add_artist::register(ctx, ready)?,
        availability_check::register(ctx, ready)?,
        close::register(ctx, ready)?,
        faq::register(ctx, ready)?,
        fetch_suggestions::register(ctx, ready)?,
        fixed::register(ctx, ready)?,
        get_discord_role::register(ctx, ready)?,
        goodmorning::register(ctx, ready)?,
        goodnight::register(ctx, ready)?,
        image::register(ctx, ready)?,
        open::register(ctx, ready)?,
        reputation::register(ctx, ready)?,
        saves::register(ctx, ready)?,
        spoilers::register(ctx, ready)?,
        support::register(ctx, ready)?,
    ];

    Ok(commands)
}
