use serenity::all::{Context, CreateCommand, Ready};

use crate::Result;

pub mod levels;
pub mod member_count;
pub mod ping;
pub mod rank;
pub mod rule;
pub mod scam;
pub mod server_info;
pub mod xp;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![
        levels::register(ctx, ready)?,
        member_count::register(ctx, ready)?,
        ping::register(ctx, ready)?,
        rank::register(ctx, ready)?,
        rule::register(ctx, ready)?,
        scam::register(ctx, ready)?,
        server_info::register(ctx, ready)?,
        xp::register(ctx, ready)?,
    ];

    Ok(commands)
}
