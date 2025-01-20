use serenity::all::{Context, CreateCommand, Ready};

use crate::Result;

pub mod prefix_commands;
pub mod slash_commands;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    slash_commands::register(ctx, ready)
}
