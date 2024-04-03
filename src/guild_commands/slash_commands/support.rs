use crate::sqlx_lib::{
    create_support_faq, delete_support_faq, get_all_support_faq, get_support_answer,
};
use crate::utils::{message_response, parse_options, send_embed};
use crate::{Error, Result};
use serenity::all::{
    Command, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, GuildId, Permissions, ResolvedValue,
};

async fn get(
    ctx: &Context,
    interaction: &CommandInteraction,
    guild_id: GuildId,
    support_id: &str,
) -> Result<()> {
    let answer = get_support_answer(guild_id.get() as i64, &support_id.to_lowercase()).await?;

    send_embed(
        ctx,
        interaction,
        CreateMessage::new().embed(CreateEmbed::new().title(support_id).description(answer)),
    )
    .await?;

    Ok(())
}

async fn add(
    ctx: &Context,
    interaction: &CommandInteraction,
    guild_id: GuildId,
    support_id: &str,
    answer: &str,
) -> Result<()> {
    create_support_faq(guild_id.get() as i64, &support_id.to_lowercase(), answer).await?;

    message_response(ctx, interaction, "Support info added").await?;

    Ok(())
}

async fn list(ctx: &Context, interaction: &CommandInteraction, guild_id: GuildId) -> Result<()> {
    let faqs = get_all_support_faq(guild_id.get() as i64).await?;

    if faqs.is_empty() {
        message_response(ctx, interaction, "No support for this server").await?;
        return Ok(());
    }

    let ids: Vec<String> = faqs.into_iter().map(|faq| faq.id).collect();

    send_embed(
        ctx,
        interaction,
        CreateMessage::new().embed(
            CreateEmbed::new()
                .title("Support IDs")
                .description(ids.join("\n")),
        ),
    )
    .await?;

    Ok(())
}

async fn remove(
    ctx: &Context,
    interaction: &CommandInteraction,
    guild_id: GuildId,
    support_id: &str,
) -> Result<()> {
    delete_support_faq(guild_id.get() as i64, &support_id.to_lowercase()).await?;

    message_response(ctx, interaction, "Support info removed").await?;

    Ok(())
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let command = &interaction.data.options()[0];

    let options = match &command.value {
        ResolvedValue::SubCommand(options) => options,
        _ => unreachable!("Subcommand is required"),
    };
    let options = parse_options(options);

    if command.name == "list" {
        list(ctx, interaction, guild_id).await?;
        return Ok(());
    }

    let id = match options.get("id") {
        Some(ResolvedValue::String(id)) => *id,
        _ => unreachable!("ID is required"),
    };

    match command.name {
        "get" => get(ctx, interaction, guild_id, id).await?,
        "add" => {
            let answer = match options.get("answer") {
                Some(ResolvedValue::String(answer)) => *answer,
                _ => unreachable!("Answer is required"),
            };

            add(ctx, interaction, guild_id, id, answer).await?
        }
        "remove" => remove(ctx, interaction, guild_id, id).await?,
        _ => unreachable!("Invalid subcommand"),
    };

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    let id_option = CreateCommandOption::new(
        CommandOptionType::String,
        "id",
        "The ID of the support info",
    );

    Command::create_global_command(
        ctx,
        CreateCommand::new("support")
            .description("Manage support info")
            .default_member_permissions(Permissions::MOVE_MEMBERS)
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "get",
                    "Get a support info",
                )
                .add_sub_option(id_option.clone().required(true)),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "add",
                    "Add a support info",
                )
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
            ),
    )
    .await?;

    Ok(())
}
