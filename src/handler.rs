use crate::commands::slash_commands::*;
use crate::models::ReactionRole;
use crate::sqlx_lib::get_reaction_roles;
use crate::utils::message_response;
use serenity::all::{ActivityData, Command};
use serenity::async_trait;
use serenity::model::channel::{Message, Reaction};
use serenity::model::gateway::Ready;
use serenity::model::prelude::{GuildId, Interaction, Member};
use serenity::model::user::OnlineStatus;
use serenity::prelude::{Context, EventHandler};

const COLLEGE_KINGS_GUILD_ID: u64 = 745662812335898806;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        use crate::commands::message_commands::*;
        use crate::commands::prefix_commands::*;

        if msg.author.bot {
            return;
        }

        let command = msg.content.split_whitespace().next().unwrap_or("");

        match command.to_lowercase().as_str() {
            "!ping" => ping::run(ctx, msg).await,
            _ => {
                ai_chat::run(&ctx, &msg).await;
                auto_support::run(&ctx, &msg).await;
            }
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let (reaction_roles, reaction_message, member) =
            match get_reaction_data(&ctx, &reaction).await {
                Ok(reaction_data) => reaction_data,
                Err(why) => return println!("{}", why),
            };

        for reaction_role in reaction_roles {
            if (reaction_message.id.get() != (reaction_role.message_id as u64))
                || (reaction.emoji.to_string() != reaction_role.emoji)
            {
                continue;
            }

            match member.add_role(&ctx, reaction_role.role_id as u64).await {
                Ok(_) => {
                    return;
                }
                Err(why) => {
                    println!("Cannot add role: {}", why);
                    return;
                }
            }
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let (reaction_roles, reaction_message, member) =
            match get_reaction_data(&ctx, &reaction).await {
                Ok(reaction_data) => reaction_data,
                Err(why) => return println!("{}", why),
            };

        for reaction_role in reaction_roles {
            if (reaction_message.id.get() != (reaction_role.message_id as u64))
                || (reaction.emoji.to_string() != reaction_role.emoji)
            {
                continue;
            }

            match member.remove_role(&ctx, reaction_role.role_id as u64).await {
                Ok(_) => {
                    return;
                }
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
                question::register(),
                reputation::register(),
                saves::register(),
                spoilers::register(),
                update_information_message::register(),
            ],
        )
        .await
        .expect("Failed to register slash command");

        Command::set_global_commands(
            &ctx,
            vec![
                gold_star::register(),
                infraction::register(),
                logs::register(),
                member_count::register(),
                ping::register(),
                reaction_role::register(),
                rule::register(),
                scam::register(),
                server_info::register(),
                stars::register(),
                support::register(),
            ],
        )
        .await
        .expect("Failed to register slash command");

        let activity = ActivityData::playing("College Kings");
        ctx.set_presence(Some(activity), OnlineStatus::Online);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("{} ran command: {}", command.user.tag(), command.data.name);

            command.defer_ephemeral(&ctx).await.expect("Cannot defer");

            let result = match command.data.name.as_str() {
                "add_artist" => add_artist::run(ctx, &command).await,
                "close" => close::run(ctx, &command).await,
                "fetch_suggestions" => fetch_suggestions::run(ctx, &command).await,
                "fixed" => fixed::run(ctx, &command).await,
                "get_discord_role" => get_discord_role::run(ctx, &command).await,
                "gold_star" => gold_star::run(ctx, &command).await,
                "good_morning" => good_morning::run(ctx, &command).await,
                "good_night" => good_night::run(ctx, &command).await,
                "infraction" => infraction::run(ctx, &command).await,
                "logs" => logs::run(ctx, &command).await,
                "member_count" => member_count::run(ctx, &command).await,
                "open" => open::run(ctx, &command).await,
                "patreon" => patreon::run(ctx, &command).await,
                "question" => question::run(ctx, &command).await,
                "reaction_role" => reaction_role::run(ctx, &command).await,
                "ping" => ping::run(ctx, &command).await,
                "reputation" => reputation::run(ctx, &command).await,
                "rule" => rule::run(ctx, &command).await,
                "saves" => saves::run(ctx, &command).await,
                "scam" => scam::run(ctx, &command).await,
                "server_info" => server_info::run(ctx, &command).await,
                "spoilers" => spoilers::run(ctx, &command).await,
                "stars" => stars::run(ctx, &command).await,
                "support" => support::run(ctx, &command).await,
                "update_information" => update_information_message::run(ctx, &command).await,
                _ => message_response(&ctx, &command, "Command not found").await,
            };

            if let Err(why) = result {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

async fn get_reaction_data(
    ctx: &Context,
    reaction: &Reaction,
) -> Result<(Vec<ReactionRole>, Message, Member), String> {
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

    let reaction_roles = match get_reaction_roles(guild_id.get() as i64).await {
        Ok(reaction_roles) => reaction_roles,
        Err(why) => return Err(format!("Cannot get reaction roles: {}", why)),
    };

    Ok((reaction_roles, reaction_message, member))
}
