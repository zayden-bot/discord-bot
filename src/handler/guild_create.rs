use serenity::all::{Context, Guild};
use sqlx::PgPool;

use crate::Result;

use super::Handler;

impl Handler {
    pub async fn guild_create(ctx: &Context, guild: Guild, _pool: &PgPool) -> Result<()> {
        temp_voice::events::guild_create(ctx, &guild).await;

        Ok(())
    }
}
