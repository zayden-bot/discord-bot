use serenity::all::{Channel, Context, Message};
use sqlx::PgPool;

use crate::global_commands::message_commands::*;
use crate::global_commands::prefix_commands::*;
use crate::handler::Handler;
use crate::modules::ticket::message_commands::support;
use crate::Result;

impl Handler {
    pub async fn message(ctx: &Context, msg: Message, pool: &PgPool) -> Result<()> {
        if msg.author.bot {
            return Ok(());
        }

        if let Channel::Private(c) = msg.channel(ctx).await.unwrap() {
            println!("{:?}", msg);
            println!();
            println!("{:?}", c)
        }

        let command = msg.content.split_whitespace().next().unwrap_or_default();

        match command.to_lowercase().as_str() {
            "!ping" => ping::run(ctx, msg).await?,
            "!rank" => rank::run(ctx, msg).await?,
            _ => {
                tokio::try_join!(levels::run(ctx, &msg), support(ctx, &msg, pool))?;
            }
        }

        Ok(())
    }
}
