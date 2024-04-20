use serenity::all::Context;

use crate::Result;

pub mod message_commands;
pub mod prefix_commands;
pub mod slash_commands;

pub async fn register(ctx: &Context) -> Result<()> {
    slash_commands::register(ctx).await?;

    println!("Global commands registered!");

    Ok(())
}
