use serenity::all::Context;

use crate::Result;

pub mod add_artist;
pub mod close;
pub mod fetch_suggestions;
pub mod fixed;
pub mod get_discord_role;
pub mod good_morning;
pub mod good_night;
pub mod image;
pub mod link;
pub mod open;
pub mod patreon;
pub mod reputation;
pub mod saves;
pub mod spoilers;

pub async fn register(ctx: &Context) -> Result<()> {
    add_artist::register(ctx).await?;
    close::register(ctx).await?;
    fetch_suggestions::register(ctx).await?;
    fixed::register(ctx).await?;
    get_discord_role::register(ctx).await?;
    good_morning::register(ctx).await?;
    good_night::register(ctx).await?;
    image::register(ctx).await?;
    link::register(ctx).await?;
    open::register(ctx).await?;
    patreon::register(ctx).await?;
    reputation::register(ctx).await?;
    saves::register(ctx).await?;
    spoilers::register(ctx).await?;

    Ok(())
}
