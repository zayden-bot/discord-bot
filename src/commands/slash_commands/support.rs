use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::Permissions;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use crate::sqlx_lib::{create_support_faq, delete_support_faq, get_all_support_faq, get_support_answer};

fn get_support_id(subcommand: &CommandDataOption) -> Result<&String, &str> {
    match subcommand.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::String(support_id)) => Ok(support_id),
        _ => Err("Invalid support ID"),
    }
}

async fn get<'a>(_ctx: &Context, subcommand: &CommandDataOption, guild_id: GuildId, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let support_id = match get_support_id(subcommand) {
        Ok(support_id) => support_id,
        Err(err) => {
            response.interaction_response_data(|message| message.content(err));
            return response;
        }
    };

    let answer = match get_support_answer(guild_id.0 as i64, &support_id.to_lowercase()).await {
        Ok(answer) => answer,
        Err(_) => {
            response.interaction_response_data(|message| message.content(format!("Support ID: `{}` not found", support_id)));
            return response;
        },
    };

    response.interaction_response_data(|message| message.embed(|e| {
        e.title(support_id)
            .description(answer)
    }));

    response
}

async fn add<'a>(_ctx: &Context, subcommand: &CommandDataOption, guild_id: GuildId, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let support_id = match get_support_id(subcommand) {
        Ok(support_id) => support_id,
        Err(err) => {
            response.interaction_response_data(|message| message.content(err));
            return response;
        }
    };

    let answer = if let Some(CommandDataOptionValue::String(answer)) = subcommand.options[1].resolved.as_ref() {
        answer
    } else {
        response.interaction_response_data(|message| message.content("Invalid support answer"));
        return response;
    };

    match create_support_faq(guild_id.0 as i64, &support_id.to_lowercase(), answer).await {
        Ok(_) => {},
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error adding support info"));
            return response;
        },
    };

    response.interaction_response_data(|message| message.content(format!("Add support info with ID `{}`", support_id)));
    response
}

async fn list<'a>(_ctx: &Context, guild_id: GuildId, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let faqs = match get_all_support_faq(guild_id.0 as i64).await {
        Ok(faqs) => faqs,
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error getting support info"));
            return response;
        },
    };

    if faqs.is_empty() {
        response.interaction_response_data(|message| message.content("No support info found"));
        return response;
    }

    let ids = faqs.into_iter().map(|faq| faq.id).collect::<Vec<String>>();

    response.interaction_response_data(|message| message.content(format!("```{}```", ids.join("\n"))));
    response
}

async fn remove<'a>(_ctx: &Context, subcommand: &CommandDataOption, guild_id: GuildId, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let support_id = match get_support_id(subcommand) {
        Ok(support_id) => support_id,
        Err(err) => {
            response.interaction_response_data(|message| message.content(err));
            return response;
        }
    };

    match delete_support_faq(guild_id.0 as i64, &support_id.to_lowercase()).await {
        Ok(_) => {},
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error removing support info"));
            return response;
        },
    };

    response.interaction_response_data(|message| message.content(format!("Removed support info with ID `{}`", support_id)));
    response
}

pub async fn run<'a>(_ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            response.interaction_response_data(|message| message.content("This command can only be used in a server"));
            return response;
        },
    };

    let subcommand = &interaction.data.options[0];
    return match subcommand.name.as_str() {
        "get" => get(_ctx, subcommand, guild_id, response).await,
        "add" => add(_ctx, subcommand, guild_id, response).await,
        "list" => list(_ctx, guild_id, response).await,
        "remove" => remove(_ctx, subcommand, guild_id, response).await,
        _ => {
            response.interaction_response_data(|message| message.content("Invalid subcommand"));
            response
        },
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("support")
        .description("A ping command")
        .default_member_permissions(Permissions::MOVE_MEMBERS)
        .create_option(|option|
            option.name("get")
                .description("Get a support info")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option|
                    sub_option.name("id")
                        .description("The ID of the support info")
                        .kind(CommandOptionType::String)
                        .required(true)))
        .create_option(|option|
            option.name("add")
                .description("Add a support info")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option|
                    sub_option.name("id")
                        .description("The ID of the support info")
                        .kind(CommandOptionType::String)
                        .required(true))
                .create_sub_option(|sub_option|
                    sub_option.name("answer")
                        .description("The answer of the support info")
                        .kind(CommandOptionType::String)
                        .required(true)))
        .create_option(|option|
            option.name("list")
                .description("Get a list of valid support IDs")
                .kind(CommandOptionType::SubCommand))
        .create_option(|option|
            option.name("remove")
                .description("Remove an existing support ID")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option|
                    sub_option.name("id")
                        .description("The ID of the support info")
                        .kind(CommandOptionType::String)
                        .required(true)))
}