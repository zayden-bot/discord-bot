use serenity::all::{Context, Guild};
use sqlx::PgPool;
use zayden_core::SlashCommand;
use zayden_core::cache::GuildMembersCache;

use crate::modules;
use crate::modules::events::live::Live;
use crate::{BRADSTER_GUILD, Result};

use super::Handler;

impl Handler {
    pub async fn guild_create(ctx: &Context, guild: Guild, _pool: &PgPool) -> Result<()> {
        let (_, _, r) = tokio::join!(
            temp_voice::events::guild_create(ctx, &guild),
            GuildMembersCache::guild_create(ctx, &guild),
            guild.set_commands(ctx, modules::global_register(ctx)),
        );

        r?;

        if guild.id == BRADSTER_GUILD {
            guild
                .create_command(ctx, Live::register(ctx).unwrap())
                .await
                .unwrap();
        }

        Ok(())
    }
}
