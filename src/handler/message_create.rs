use serenity::all::{Context, Message};
use sqlx::PgPool;
use zayden_core::MessageCommand;

use crate::Result;
use crate::handler::Handler;
use crate::modules::levels::Levels;
use crate::modules::ticket::message_commands::support;

impl Handler {
    pub async fn message_create(ctx: &Context, msg: Message, pool: &PgPool) -> Result<()> {
        if msg.author.bot {
            return Ok(());
        }

        tokio::try_join!(Levels::run(ctx, &msg, pool), support(ctx, &msg, pool))?;

        Ok(())
    }
}
