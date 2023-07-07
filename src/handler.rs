use crate::commands::slash_commands::*;
use serenity::async_trait;
use serenity::builder::CreateInteractionResponse;
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::prelude::{Interaction, InteractionResponseType};
use serenity::model::prelude::command::Command;
use serenity::model::user::OnlineStatus;
use serenity::prelude::{Context, EventHandler};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        use crate::commands::message_commands::*;
        use crate::commands::prefix_commands::*;

        if msg.author.bot {
            return;
        }

        let command = match msg.content.split_whitespace().next() {
            Some(command) => command,
            None => return,
        };

        match command.to_lowercase().as_str() {
            "!create_qr_code" => create_qr_code::run(ctx, msg).await,
            "!ping" => ping::run(ctx, msg).await,
            "!gm" => gm::run(ctx, msg).await,
            "!gn" => gn::run(ctx, msg).await,
            _ => {
                auto_support::run(&ctx, &msg).await;
                ai_chat::run(&ctx, &msg).await;
            },
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_application_commands(&ctx, |command| {
            command
                .create_application_command(|command| get_discord_role::register(command))
                .create_application_command(|command| gold_star::register(command))
                .create_application_command(|command| ping::register(command))
                .create_application_command(|command| reputation::register(command))
                .create_application_command(|command| stars::register(command))
        }).await.expect("Failed to register slash command");

        let activity = Activity::watching("for the chosen one");
        ctx.set_presence(Some(activity), OnlineStatus::Online).await;

        // TODO: Load Commands
        // TODO: Deploy Commands
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("{} ran command: {}", command.user.tag(), command.data.name);

            let mut response = CreateInteractionResponse::default();
            response.kind(InteractionResponseType::ChannelMessageWithSource);

            response = match command.data.name.as_str() {
                "get_discord_role" => get_discord_role::run(&command, response),
                "gold_star" => gold_star::run(&command, response).await,
                "ping" => ping::run(&command, response),
                "reputation" => reputation::run(&command, response),
                "stars" => stars::run(&command, response).await,
                _ => {
                    response.interaction_response_data(|message| message.content("Unknown command"));
                    response
                }
            };

            if let Err(why) = command
                .create_interaction_response(&ctx, |message| {
                    message.0 = response.0;
                    message.1 = response.1;
                    message
                }).await {
                    println!("Cannot respond to slash command: {}", why);
                }
        }
    }
}