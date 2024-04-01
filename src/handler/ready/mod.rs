mod update_information_message;
mod update_support_messages;
mod utils;

use serenity::all::{Context, OnlineStatus, Ready};

use crate::cron::start_cron_jobs;
use crate::{global_commands, guild_commands, Result};

pub async fn ready(ctx: Context, ready: Ready) -> Result<()> {
    println!("{} is connected!", ready.user.name);

    ctx.set_presence(None, OnlineStatus::Online);

    // TODO: Load Commands

    let ctx_clone = ctx.clone();
    let cron_task = tokio::spawn(async move { start_cron_jobs(ctx_clone).await });

    let tasks = tokio::join!(
        guild_commands::register(&ctx),
        global_commands::register(&ctx),
        update_messages(&ctx),
        cron_task
    );

    tasks.0?;
    tasks.1?;
    tasks.2?;
    tasks.3??;

    Ok(())
}

async fn update_messages(ctx: &Context) -> Result<()> {
    update_information_message::run(ctx).await?;
    update_support_messages::run(ctx).await?;

    Ok(())
}
