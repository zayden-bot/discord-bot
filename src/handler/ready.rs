use futures::future;
use serenity::all::{Context, OnlineStatus, Ready};

use crate::Result;
use crate::handler::Handler;
use crate::modules;

impl Handler {
    pub async fn ready(ctx: &Context, ready: Ready) -> Result<()> {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(None, OnlineStatus::Online);

        let futures = ready.guilds.iter().map(|guild| {
            guild
                .id
                .set_commands(ctx, modules::global_register(ctx, &ready))
        });
        future::try_join_all(futures).await.unwrap();

        Ok(())
    }
}
