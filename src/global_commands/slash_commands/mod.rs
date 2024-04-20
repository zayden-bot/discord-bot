use serenity::all::{Command, Context};

use crate::Result;

pub mod gold_star;
pub mod infraction;
pub mod levels;
pub mod logs;
pub mod member_count;
pub mod ping;
pub mod rank;
pub mod reaction_role;
pub mod rule;
pub mod scam;
pub mod server_info;
pub mod stars;
pub mod xp;

pub async fn register(ctx: &Context) -> Result<()> {
    Command::set_global_commands(
        ctx,
        vec![
            gold_star::register(),
            infraction::register(),
            levels::register(),
            logs::register(),
            member_count::register(),
            ping::register(),
            rank::register(),
            reaction_role::register(),
            rule::register(),
            scam::register(),
            server_info::register(),
            stars::register(),
            xp::register(),
        ],
    )
    .await?;

    Ok(())
}
