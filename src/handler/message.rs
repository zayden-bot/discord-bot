use serenity::all::Context;
use serenity::all::Message;

use crate::global_commands::message_commands::*;
use crate::global_commands::prefix_commands::*;
use crate::Result;

pub async fn message(ctx: Context, msg: Message) -> Result<()> {
    if msg.author.bot {
        return Ok(());
    }

    let command = msg.content.split_whitespace().next().unwrap_or("");

    match command.to_lowercase().as_str() {
        "!ping" => ping::run(ctx, msg).await?,
        "!rank" => rank::run(ctx, msg).await?,
        _ => {
            tokio::join!(
                ai_chat::run(&ctx, &msg),
                auto_support::run(&ctx, &msg),
                levels::run(&ctx, &msg)
            )
            .1?;
        }
    }

    Ok(())
}
