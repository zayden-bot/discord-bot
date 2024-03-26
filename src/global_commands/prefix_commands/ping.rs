use serenity::all::{Context, Message};

use crate::Result;

pub async fn run(ctx: Context, msg: Message) -> Result<()> {
    msg.channel_id.say(&ctx, "Pong!").await?;

    Ok(())
}
