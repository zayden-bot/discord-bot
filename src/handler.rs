use crate::commands::message_commands::*;
use crate::commands::prefix_commands::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::user::OnlineStatus;
use serenity::prelude::{Context, EventHandler};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let command = match msg.content.split_whitespace().next() {
            Some(command) => command,
            None => return,
        };

        match command.to_lowercase().as_str() {
            "!ping" => ping::run(ctx, msg).await,
            "!gm" => gm::run(ctx, msg).await,
            "!gn" => gn::run(ctx, msg).await,
            _ => auto_support::run(ctx, msg).await,
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let activity = Activity::watching("for the chosen one");
        ctx.set_presence(Some(activity), OnlineStatus::Online).await;

        // TODO: Load Commands
        // TODO: Deploy Commands
    }
}
