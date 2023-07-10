use serenity::builder::CreateApplicationCommand;
use serenity::model::Permissions;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use crate::sqlx_lib::{create_support_faq, delete_support_faq, get_all_support_faq, get_support_answer};
use crate::utils::{respond_with_embed, respond_with_message};

fn get_support_id(subcommand: &CommandDataOption) -> Result<&String, &str> {
    match subcommand.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::String(support_id)) => Ok(support_id),
        _ => Err("Invalid support ID"),
    }
}

async fn get(ctx: &Context, interaction: &ApplicationCommandInteraction, subcommand: &CommandDataOption, guild_id: GuildId) -> Result<(), serenity::Error> {
    let support_id = match get_support_id(subcommand) {
        Ok(support_id) => support_id,
        Err(err) => return respond_with_message(ctx, interaction, err).await,
    };

    let answer = match get_support_answer(guild_id.0 as i64, &support_id.to_lowercase()).await {
        Ok(answer) => answer,
        Err(_) => return respond_with_message(ctx, interaction, "Error getting support info").await,
    };

    respond_with_embed(ctx, interaction, |e| {
        e.title(support_id)
            .description(answer)
    }).await
}

async fn add(ctx: &Context, interaction: &ApplicationCommandInteraction, subcommand: &CommandDataOption, guild_id: GuildId) -> Result<(), serenity::Error> {
    let support_id = match get_support_id(subcommand) {
        Ok(support_id) => support_id,
        Err(err) => return respond_with_message(ctx, interaction, err).await,
    };

    let answer = match subcommand.options[1].resolved.as_ref() {
        Some(CommandDataOptionValue::String(answer)) => answer,
        _ => return respond_with_message(ctx, interaction, "Invalid answer").await,
    };

    match create_support_faq(guild_id.0 as i64, &support_id.to_lowercase(), answer).await {
        Ok(_) => {},
        Err(_) => return respond_with_message(ctx, interaction, "Error adding support info").await,
    };

    respond_with_message(ctx, interaction, "Support info added").await
}

async fn list(ctx: &Context, interaction: &ApplicationCommandInteraction, guild_id: GuildId) -> Result<(), serenity::Error> {
    let faqs = match get_all_support_faq(guild_id.0 as i64).await {
        Ok(faqs) => faqs,
        Err(_) => return respond_with_message(ctx, interaction, "Error getting support info").await,
    };

    if faqs.is_empty() {
        return respond_with_message(ctx, interaction, "No support for this server").await;
    }

    let ids = faqs.into_iter().map(|faq| faq.id).collect::<Vec<String>>();

    respond_with_embed(ctx, interaction, |e| {
        e.title("Support IDs")
            .description(ids.join("\n"))
    }).await
}

async fn remove(ctx: &Context, interaction: &ApplicationCommandInteraction, subcommand: &CommandDataOption, guild_id: GuildId) -> Result<(), serenity::Error> {
    let support_id = match get_support_id(subcommand) {
        Ok(support_id) => support_id,
        Err(err) => return respond_with_message(ctx, interaction, err).await,
    };

    match delete_support_faq(guild_id.0 as i64, &support_id.to_lowercase()).await {
        Ok(_) => {},
        Err(_) => return respond_with_message(ctx, interaction, "Error removing support info").await,
    };

    respond_with_message(ctx, interaction, "Support info removed").await
}

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let subcommand = &interaction.data.options[0];
    return match subcommand.name.as_str() {
        "get" => get(ctx, interaction, subcommand, guild_id).await,
        "add" => add(ctx, interaction, subcommand, guild_id).await,
        "list" => list(ctx, interaction, guild_id).await,
        "remove" => remove(ctx, interaction, subcommand, guild_id).await,
        _ => respond_with_message(ctx, interaction, "Invalid subcommand").await,
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