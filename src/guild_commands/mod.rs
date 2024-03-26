use serenity::all::Context;

use crate::Result;

pub mod slash_commands;

pub async fn register(ctx: &Context) -> Result<()> {
    slash_commands::register(ctx).await?;

    Ok(())
}
