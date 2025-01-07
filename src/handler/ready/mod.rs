use async_trait::async_trait;
use futures::future;
use serenity::all::{Context, OnlineStatus, Ready};

use message_updates::update_messages;

use crate::cron::start_cron_jobs;
use crate::handler::Handler;
use crate::modules;
use crate::modules::misc::Sleep;
use crate::{global_commands, guilds, Result};

mod message_updates;

impl Handler {
    pub async fn ready(ctx: &Context, ready: Ready) -> Result<()> {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(None, OnlineStatus::Online);

        // TODO: Load Commands
        let mut command_map = guilds::commands(ctx, &ready)?;

        let futures = ready.guilds.iter().map(|guild| {
            let mut commands = command_map.remove(&guild.id).unwrap_or_default();
            commands.extend(
                [
                    global_commands::register(ctx, &ready).unwrap(),
                    modules::global_register(ctx, &ready).unwrap(),
                ]
                .concat(),
            );

            guild.id.set_commands(ctx, commands)
        });
        future::try_join_all(futures).await?;

        update_messages(ctx).await?;

        let ctx_clone = ctx.clone();
        tokio::spawn(async move { Sleep::on_ready(ctx_clone, ready).await });

        let ctx_clone = ctx.clone();
        tokio::spawn(async move { start_cron_jobs(ctx_clone).await });

        Ok(())
    }
}

#[async_trait]
pub trait OnReady {
    async fn on_ready(ctx: Context, ready: Ready) -> Result<()>;
}
