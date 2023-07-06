use crate::commands::slash_commands::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{Interaction, InteractionResponseType};
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
            _ => {
                auto_support::run(&ctx, &msg).await;
                ai_chat::run(&ctx, &msg).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_application_commands(&ctx, |command| {
            command
                .create_application_command(|command| gold_star::register(command))
                .create_application_command(|command| good_morning::register(command))
                .create_application_command(|command| good_night::register(command))
                .create_application_command(|command| ping::register(command))
        })
        .await
        .expect("Failed to register slash command");

        let activity = Activity::playing("College Kings");
        ctx.set_presence(Some(activity), OnlineStatus::Online).await;

        // TODO: Load Commands
        // TODO: Deploy Commands
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("{} ran command: {}", command.user.tag(), command.data.name);

            let context = match command.data.name.as_str() {
                "gold_star" => gold_star::run(&command).await,
                "good_morning" => good_morning::run(&command).await,
                "good_night" => good_night::run(&command).await,
                "ping" => ping::run(&command),
                _ => "Unknown command".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(context))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}
