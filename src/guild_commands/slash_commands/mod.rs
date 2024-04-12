use serenity::all::Context;

use crate::Result;

pub mod add_artist;
pub mod availability_check;
pub mod close;
pub mod faq;
pub mod fetch_suggestions;
pub mod fixed;
pub mod get_discord_role;
pub mod goodmorning;
pub mod goodnight;
pub mod image;
pub mod link;
pub mod open;
pub mod patreon;
pub mod reputation;
pub mod review;
pub mod saves;
pub mod spoilers;
pub mod support;
pub mod test;

pub async fn register(ctx: &Context) -> Result<()> {
    tokio::try_join!(
        add_artist::register(ctx),
        availability_check::register(ctx),
        close::register(ctx),
        faq::register(ctx),
        fetch_suggestions::register(ctx),
        fixed::register(ctx),
        get_discord_role::register(ctx),
        goodmorning::register(ctx),
        goodnight::register(ctx),
        image::register(ctx),
        link::register(ctx),
        open::register(ctx),
        patreon::register(ctx),
        reputation::register(ctx),
        review::register(ctx),
        saves::register(ctx),
        spoilers::register(ctx),
        support::register(ctx),
        test::register(ctx),
    )?;

    Ok(())
}
