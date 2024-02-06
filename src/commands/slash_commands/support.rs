use crate::sqlx_lib::{
    create_support_faq, delete_support_faq, get_all_support_faq, get_support_answer,
};
use crate::utils::{message_response, send_embed};
use serenity::all::{
    CommandDataOption, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context,
    CreateCommand, CreateCommandOption, CreateEmbed, CreateMessage, GuildId, Message, Permissions,
};

async fn get(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[CommandDataOption],
    guild_id: GuildId,
) -> Result<Message, serenity::Error> {
    let support_id = match options[0].value.as_str() {
        Some(support_id) => support_id,
        None => return message_response(ctx, interaction, "Invalid support ID").await,
    };

    let answer = match get_support_answer(guild_id.get() as i64, &support_id.to_lowercase()).await {
        Ok(answer) => answer,
        Err(_) => return message_response(ctx, interaction, "Error getting support info").await,
    };

    send_embed(
        ctx,
        interaction,
        CreateMessage::new().embed(CreateEmbed::new().title(support_id).description(answer)),
    )
    .await
}

async fn add(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[CommandDataOption],
    guild_id: GuildId,
) -> Result<Message, serenity::Error> {
    let support_id = match options[0].value.as_str() {
        Some(support_id) => support_id,
        None => return message_response(ctx, interaction, "Invalid support ID").await,
    };

    let answer = match options[1].value.as_str() {
        Some(answer) => answer,
        None => return message_response(ctx, interaction, "Invalid answer").await,
    };

    if create_support_faq(guild_id.get() as i64, &support_id.to_lowercase(), answer)
        .await
        .is_err()
    {
        return message_response(ctx, interaction, "Error adding support info").await;
    }

    message_response(ctx, interaction, "Support info added").await
}

async fn list(
    ctx: &Context,
    interaction: &CommandInteraction,
    guild_id: GuildId,
) -> Result<Message, serenity::Error> {
    let faqs = match get_all_support_faq(guild_id.get() as i64).await {
        Ok(faqs) => faqs,
        Err(_) => return message_response(ctx, interaction, "Error getting support info").await,
    };

    if faqs.is_empty() {
        return message_response(ctx, interaction, "No support for this server").await;
    }

    let ids = faqs.into_iter().map(|faq| faq.id).collect::<Vec<String>>();

    send_embed(
        ctx,
        interaction,
        CreateMessage::new().embed(
            CreateEmbed::new()
                .title("Support IDs")
                .description(ids.join("\n")),
        ),
    )
    .await
}

async fn remove(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[CommandDataOption],
    guild_id: GuildId,
) -> Result<Message, serenity::Error> {
    let support_id = match options[0].value.as_str() {
        Some(support_id) => support_id,
        None => return message_response(ctx, interaction, "Invalid support ID").await,
    };

    if delete_support_faq(guild_id.get() as i64, &support_id.to_lowercase())
        .await
        .is_err()
    {
        return message_response(ctx, interaction, "Error removing support info").await;
    }

    message_response(ctx, interaction, "Support info removed").await
}

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return message_response(
                &ctx,
                interaction,
                "This command can only be used in a server",
            )
            .await
        }
    };

    let command = &interaction.data.options[0];

    let options = match &command.value {
        CommandDataOptionValue::SubCommand(options) => options,
        _ => return message_response(&ctx, interaction, "Invalid subcommand").await,
    };

    match command.name.as_str() {
        "get" => get(&ctx, interaction, options, guild_id).await,
        "add" => add(&ctx, interaction, options, guild_id).await,
        "list" => list(&ctx, interaction, guild_id).await,
        "remove" => remove(&ctx, interaction, options, guild_id).await,
        _ => message_response(&ctx, interaction, "Invalid subcommand").await,
    }
}

pub fn register() -> CreateCommand {
    let id_option = CreateCommandOption::new(
        CommandOptionType::String,
        "id",
        "The ID of the support info",
    );

    CreateCommand::new("support")
        .description("Manage support info")
        .default_member_permissions(Permissions::MOVE_MEMBERS)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "get", "Get a support info")
                .add_sub_option(id_option.clone().required(true)),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "add", "Add a support info")
                .add_sub_option(id_option.clone().required(true))
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "answer",
                        "The answer of the support info",
                    )
                    .required(true),
                ),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "list",
            "Get a list of valid support IDs",
        ))
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "remove",
                "Remove an existing support ID",
            )
            .add_sub_option(id_option.required(true)),
        )
}
