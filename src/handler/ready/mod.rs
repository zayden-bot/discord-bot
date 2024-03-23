mod update_information_message;
mod update_support_messages;
mod utils;

use serenity::all::{ActivityData, Command, Context, GuildId, OnlineStatus, Ready};

use crate::commands::slash_commands::*;
use crate::cron::start_cron_jobs;
use crate::{Result, COLLEGE_KINGS_GUILD_ID};

pub async fn ready(ctx: Context, ready: Ready) -> Result<()> {
    println!("{} is connected!", ready.user.name);

    // TODO: Load Commands

    // Deploy Commands
    GuildId::set_commands(
        GuildId::new(COLLEGE_KINGS_GUILD_ID),
        &ctx,
        vec![
            add_artist::register(),
            close::register(),
            fetch_suggestions::register(),
            fixed::register(),
            get_discord_role::register(),
            open::register(),
            patreon::register(),
            good_morning::register(),
            good_night::register(),
            image::register(),
            link::register(),
            reputation::register(),
            saves::register(),
            spoilers::register(),
        ],
    )
    .await?;

    Command::set_global_commands(
        &ctx,
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
            support::register(),
            xp::register(),
        ],
    )
    .await?;

    let activity = ActivityData::playing("College Kings");
    ctx.set_presence(Some(activity), OnlineStatus::Online);

    update_messages(&ctx).await?;

    tokio::spawn(async move { start_cron_jobs(&ctx).await });

    Ok(())
}

async fn update_messages(ctx: &Context) -> Result<()> {
    update_information_message::run(ctx).await?;
    update_support_messages::run(ctx).await?;

    Ok(())
}
