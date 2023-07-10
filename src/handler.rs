use crate::commands::slash_commands::*;
use serenity::async_trait;
use serenity::model::channel::{Message, Reaction};
use serenity::model::gateway::{Activity, Ready};
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{Interaction, Member};
use serenity::model::user::OnlineStatus;
use serenity::prelude::{Context, EventHandler};
use crate::models::ReactionRole;
use crate::sqlx_lib::get_reaction_roles;
use crate::utils::respond_with_message;

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

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let (reaction_roles, reaction_message, mut member) = match get_reaction_data(&ctx, &reaction).await {
            Ok(reaction_data) => reaction_data,
            Err(why) => return println!("{}", why),
        };

        for reaction_role in reaction_roles {
            if (reaction_message.id.0 != reaction_role.message_id as u64) || (reaction.emoji.to_string() != reaction_role.emoji) {
                continue;
            }

            match member.add_role(&ctx, reaction_role.role_id as u64).await {
                Ok(_) => { return; }
                Err(why) => {
                    println!("Cannot add role: {}", why);
                    return;
                }
            }
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let (reaction_roles, reaction_message, mut member) = match get_reaction_data(&ctx, &reaction).await {
            Ok(reaction_data) => reaction_data,
            Err(why) => return println!("{}", why),
        };

        for reaction_role in reaction_roles {
            if (reaction_message.id.0 != reaction_role.message_id as u64) || (reaction.emoji.to_string() != reaction_role.emoji) {
                continue;
            }

            match member.remove_role(&ctx, reaction_role.role_id as u64).await {
                Ok(_) => { return; }
                Err(why) => {
                    println!("Cannot remove role: {}", why);
                    return;
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // TODO: Load Commands

        // Deploy Commands
        Command::set_global_application_commands(&ctx, |command| {
            command
                .create_application_command(|command| answer::register(command))
                .create_application_command(|command| fetch_suggestions::register(command))
                .create_application_command(|command| get_discord_role::register(command))
                .create_application_command(|command| gold_star::register(command))
                .create_application_command(|command| member_count::register(command))
                .create_application_command(|command| patreon::register(command))
                .create_application_command(|command| good_morning::register(command))
                .create_application_command(|command| good_night::register(command))
                .create_application_command(|command| ping::register(command))
                .create_application_command(|command| question::register(command))
                .create_application_command(|command| reputation::register(command))
                .create_application_command(|command| rule::register(command))
                .create_application_command(|command| saves::register(command))
                .create_application_command(|command| scam::register(command))
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

            let result = match command.data.name.as_str() {
                "answer" => answer::run(&ctx, &command).await,
                "fetch_suggestions" => fetch_suggestions::run(&ctx, &command).await,
                "get_discord_role" => get_discord_role::run(&ctx, &command).await,
                "gold_star" => gold_star::run(&ctx, &command).await,
                "good_morning" => good_morning::run(&ctx, &command).await,
                "good_night" => good_night::run(&ctx, &command).await,
                "member_count" => member_count::run(&ctx, &command).await,
                "patreon" => patreon::run(&ctx, &command).await,
                "question" => question::run(&ctx, &command).await,
                "ping" => ping::run(&ctx, &command).await,
                "reputation" => reputation::run(&ctx, &command).await,
                "rule" => rule::run(&ctx, &command).await,
                "saves" => saves::run(&ctx, &command).await,
                "scam" => scam::run(&ctx, &command).await,
                "server_info" => server_info::run(&ctx, &command).await,
                "spoilers" => spoilers::run(&ctx, &command).await,
                "stars" => stars::run(&ctx, &command).await,
                "support" => support::run(&ctx, &command).await,
                _ => respond_with_message(&ctx, &command, "Command not found").await,
            };

            if let Err(why) = result {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

async fn get_reaction_data(ctx: &Context, reaction: &Reaction) -> Result<(Vec<ReactionRole>, Message, Member), String> {
    let guild_id = match reaction.guild_id {
        Some(guild_id) => guild_id,
        None => return Err("Cannot get guild id".to_string()),
    };

    let reaction_message = match reaction.message(&ctx).await {
        Ok(reaction_message) => reaction_message,
        Err(why) => return Err(format!("Cannot get reaction message: {}", why)),
    };

    let user_id = match reaction.user_id {
        Some(user_id) => user_id,
        None => return Err("Cannot get user id".to_string()),
    };

    let member = match guild_id.member(&ctx, user_id).await {
        Ok(member) => member,
        Err(why) => return Err(format!("Cannot get member: {}", why)),
    };

    let reaction_roles = match get_reaction_roles(guild_id.0 as i64).await {
        Ok(reaction_roles) => reaction_roles,
        Err(why) => return Err(format!("Cannot get reaction roles: {}", why)),
    };

    Ok((reaction_roles, reaction_message, member))
}