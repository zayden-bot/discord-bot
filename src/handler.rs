use crate::commands::slash_commands::*;
use serenity::async_trait;
use serenity::builder::CreateInteractionResponse;
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
            "!gm" => gm::run(ctx, msg).await,
            "!gn" => gn::run(ctx, msg).await,
            _ => {
                ai_chat::run(&ctx, &msg).await;
                auto_support::run(&ctx, &msg).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // TODO: Load Commands

        // Deploy Commands
        Command::set_global_application_commands(&ctx, |command| {
            command
                .create_application_command(|command| get_discord_role::register(command))
                .create_application_command(|command| gold_star::register(command))
                .create_application_command(|command| member_count::register(command))
                .create_application_command(|command| patreon::register(command))
                .create_application_command(|command| good_morning::register(command))
                .create_application_command(|command| good_night::register(command))
                .create_application_command(|command| ping::register(command))
                .create_application_command(|command| reputation::register(command))
                .create_application_command(|command| saves::register(command))
                .create_application_command(|command| server_info::register(command))
                .create_application_command(|command| spoilers::register(command))
                .create_application_command(|command| stars::register(command))
                .create_application_command(|command| support::register(command))
        })
        .await
        .expect("Failed to register slash command");

        let mut activity = Activity::playing("College Kings");
        activity.url = Some("https://www.patreon.com/collegekings".parse().unwrap());
        ctx.set_presence(Some(activity), OnlineStatus::Online).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("{} ran command: {}", command.user.tag(), command.data.name);

            let mut response = CreateInteractionResponse::default();
            response.kind(InteractionResponseType::ChannelMessageWithSource);

            response = match command.data.name.as_str() {
                "get_discord_role" => get_discord_role::run(&ctx, &command, response),
                "gold_star" => gold_star::run(&ctx, &command, response).await,
                "good_morning" => good_morning::run(&ctx, &command, response).await,
                "good_night" => good_night::run(&ctx, &command, response).await,
                "member_count" => member_count::run(&ctx, &command, response),
                "patreon" => patreon::run(&ctx, &command, response),
                "ping" => ping::run(&ctx, &command, response),
                "reputation" => reputation::run(&ctx, &command, response),
                "saves" => saves::run(&ctx, &command, response).await,
                "server_info" => server_info::run(&ctx, &command, response),
                "spoilers" => spoilers::run(&ctx, &command, response).await,
                "stars" => stars::run(&ctx, &command, response).await,
                "support" => support::run(&ctx, &command, response).await,
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
