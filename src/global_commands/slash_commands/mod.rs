use serenity::all::Context;

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
    tokio::try_join!(
        gold_star::register(ctx),
        infraction::register(ctx),
        levels::register(ctx),
        logs::register(ctx),
        member_count::register(ctx),
        ping::register(ctx),
        rank::register(ctx),
        reaction_role::register(ctx),
        rule::register(ctx),
        scam::register(ctx),
        server_info::register(ctx),
        stars::register(ctx),
        xp::register(ctx),
    )?;

    Ok(())
}
