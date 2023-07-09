use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::Permissions;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;

pub async fn run<'a>(ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            response.interaction_response_data(|message| message.content("This command can only be used in a server"));
            return response;
        }
    };

    let user = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::User(user, _member)) => user,
        _ => {
            response.interaction_response_data(|message| message.content("Invalid user"));
            return response;
        }
    };

    let reason = match interaction.data.options.get(1) {
        Some(reason) => {
            match reason.resolved.as_ref() {
                Some(CommandDataOptionValue::String(reason)) => reason,
                _ => {
                    response.interaction_response_data(|message| message.content("Invalid reason"));
                    return response;
                }
            }
        },
        _ => "Compromised account: Sending scam links."
    };

    let member = match guild_id.member(&ctx, &user.id).await {
        Ok(member) => member,
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error getting member"));
            return response;
        }
    };

    let _ = user.dm(&ctx, |message| {
        message.embed(|e| {
            e.description(format!("You have been soft banned from {} for the following reason: {}", guild_id.name(&ctx).unwrap(), reason))
        })
    }).await;

    match guild_id.ban_with_reason(&ctx, user.id, 1, reason).await {
        Ok(_) => {},
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error banning member"));
            return response;
        }
    };

    response.interaction_response_data(|message| message.embed(|e| {
        e.title("Soft Banned")
            .description(format!("{} has been successfully soft banned for the following reason: {}", member.user.name, reason))
    }));
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("scam")
        .description("Soft ban a compromised account")
        .default_member_permissions(Permissions::KICK_MEMBERS)
        .create_option(|option|
            option.name("member")
                .description("Member to soft ban")
                .kind(CommandOptionType::User)
                .required(true))
        .create_option(|option|
            option.name("reason")
                .description("Reason for soft ban")
                .kind(CommandOptionType::String))
}