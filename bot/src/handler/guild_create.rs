use serenity::all::{Context, Guild};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;
use zayden_core::cache::GuildMembersCache;

use crate::modules;
use crate::modules::events::live::Live;
use crate::modules::lfg::{GuildTable, PostTable};
use crate::{BRADSTER_GUILD, Result};

use super::Handler;

impl Handler {
    pub async fn guild_create(ctx: &Context, guild: Guild, pool: &PgPool) -> Result<()> {
        let (_, _, _, commands) = tokio::join!(
            lfg::events::guild_create::<Postgres, GuildTable, PostTable>(ctx, &guild, pool),
            temp_voice::events::guild_create(ctx, &guild),
            GuildMembersCache::guild_create(ctx, &guild),
            guild.set_commands(ctx, modules::global_register(ctx)),
        );
        commands?;

        if guild.id == BRADSTER_GUILD {
            guild
                .create_command(ctx, Live::register(ctx).unwrap())
                .await?;

            println!("Registered Bradster Guild");
        }

        Ok(())
    }
}
