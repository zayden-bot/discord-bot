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
    gold_star::register(ctx).await?;
    infraction::register(ctx).await?;
    levels::register(ctx).await?;
    logs::register(ctx).await?;
    member_count::register(ctx).await?;
    ping::register(ctx).await?;
    rank::register(ctx).await?;
    reaction_role::register(ctx).await?;
    rule::register(ctx).await?;
    scam::register(ctx).await?;
    server_info::register(ctx).await?;
    stars::register(ctx).await?;
    xp::register(ctx).await?;

    Ok(())
}
