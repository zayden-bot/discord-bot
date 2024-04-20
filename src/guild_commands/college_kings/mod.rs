use serenity::all::Context;

use crate::{guilds::college_kings::GUILD_ID, Result};

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
pub mod saves;
pub mod spoilers;
pub mod support;

pub async fn register(ctx: &Context) -> Result<()> {
    GUILD_ID
        .set_commands(
            ctx,
            vec![
                add_artist::register(),
                availability_check::register(),
                close::register(),
                faq::register(),
                fetch_suggestions::register(),
                fixed::register(),
                get_discord_role::register(),
                goodmorning::register(),
                goodnight::register(),
                image::register(),
                link::register(),
                open::register(),
                patreon::register(),
                reputation::register(),
                saves::register(),
                spoilers::register(),
                support::register(),
            ],
        )
        .await?;
    Ok(())
}
