use serenity::all::{Context, CreateCommand, Ready};

mod bunny;
// pub mod family;
pub mod gold_star;
pub mod levels;
pub mod misc;
pub mod moderation;
pub mod patreon;
pub mod reaction_roles;
pub mod ticket;

use crate::Result;

pub fn global_register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = [
        // family::register(ctx, ready)?,
        gold_star::register(ctx, ready)?,
        misc::register(ctx, ready)?,
        moderation::register(ctx, ready)?,
        patreon::register(ctx, ready)?,
        reaction_roles::register(ctx, ready)?,
    ]
    .concat();

    Ok(commands)
}
