use serenity::all::{Context, Guild};
use sqlx::PgPool;
use zayden_core::SlashCommand;

use crate::{
    BRADSTER_GUILD, Result,
    modules::{self, events::live::Live},
};

use super::Handler;

impl Handler {
    pub async fn guild_create(ctx: &Context, guild: Guild, _pool: &PgPool) -> Result<()> {
        temp_voice::events::guild_create(ctx, &guild).await;

        guild
            .set_commands(ctx, modules::global_register(ctx))
            .await
            .unwrap();

        if guild.id == BRADSTER_GUILD {
            guild
                .create_command(ctx, Live::register(ctx).unwrap())
                .await
                .unwrap();
        }

        Ok(())
    }
}
