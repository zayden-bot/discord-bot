use serenity::all::{Channel, Context, Message};
use sqlx::PgPool;
use zayden_core::MessageCommand;

use crate::global_commands::prefix_commands::{ping, rank};
use crate::handler::Handler;
use crate::modules::levels::Levels;
use crate::modules::ticket::message_commands::support;
use crate::Result;

impl Handler {
    pub async fn message(ctx: &Context, msg: Message, pool: &PgPool) -> Result<()> {
        if msg.author.bot {
            return Ok(());
        }

        let command = msg.content.split_whitespace().next().unwrap_or_default();

        match command.to_lowercase().as_str() {
            "!ping" => ping::run(ctx, msg).await?,
            "!rank" => rank::run(ctx, msg).await?,
            _ => {
                tokio::try_join!(Levels::run(ctx, &msg, pool), support(ctx, &msg, pool))?;
            }
        }

        Ok(())
    }
}
