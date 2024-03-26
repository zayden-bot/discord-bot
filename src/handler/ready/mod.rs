mod update_information_message;
mod update_support_messages;
mod utils;

use serenity::all::{Context, OnlineStatus, Ready};

use crate::cron::start_cron_jobs;
use crate::{global_commands, guild_commands, Result};

pub async fn ready(ctx: Context, ready: Ready) -> Result<()> {
    println!("{} is connected!", ready.user.name);

    // TODO: Load Commands

    // Deploy Commands
    guild_commands::register(&ctx).await?;
    global_commands::register(&ctx).await?;

    ctx.set_presence(None, OnlineStatus::Online);

    update_messages(&ctx).await?;

    tokio::spawn(async move { start_cron_jobs(ctx.clone()).await });

    Ok(())
}

async fn update_messages(ctx: &Context) -> Result<()> {
    update_information_message::run(ctx).await?;
    update_support_messages::run(ctx).await?;

    Ok(())
}
