use serenity::all::{Context, CreateCommand, Ready};
use zayden_core::SlashCommand;

pub use link::Link;
pub use sleep::Sleep;

mod link;
mod sleep;

use crate::Result;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![Sleep::register(ctx, ready)?, Link::register(ctx, ready)?];

    Ok(commands)
}
