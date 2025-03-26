use serenity::all::{Context, CreateCommand, Ready};

pub mod levels;
// pub mod moderation;
pub mod destiny2;
pub mod reaction_roles;
pub mod suggestions;
pub mod temp_voice;
pub mod ticket;

pub fn global_register(ctx: &Context, ready: &Ready) -> Vec<CreateCommand> {
    let mut cmds = destiny2::register(ctx, ready).to_vec();

    cmds.extend(levels::register(ctx, ready));
    cmds.push(reaction_roles::register(ctx, ready));
    cmds.push(suggestions::register(ctx, ready));
    cmds.push(temp_voice::register(ctx, ready));
    cmds.extend(ticket::register(ctx, ready));

    cmds
}
