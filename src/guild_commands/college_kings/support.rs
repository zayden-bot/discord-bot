use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
    EditInteractionResponse, Permissions, ResolvedValue,
};

use crate::guilds::college_kings::SUPPORT_FAQ_CHANNEL_ID;
use crate::utils::parse_options;
use crate::{Error, Result};

pub async fn list(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let menu_options: Vec<CreateSelectMenuOption> = SUPPORT_FAQ_CHANNEL_ID
        .messages_iter(ctx)
        .enumerate()
        .then(|(index, msg_result)| async move {
            let msg = msg_result?;
            let id = msg
                .content
                .lines()
                .next()
                .ok_or_else(|| Error::EmptyMessage)?
                .trim();

            Ok::<CreateSelectMenuOption, Error>(CreateSelectMenuOption::new(
                id[2..id.len() - 2].to_string(),
                index.to_string(),
            ))
        })
        .try_collect()
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().select_menu(CreateSelectMenu::new(
                "support_faq",
                CreateSelectMenuKind::String {
                    options: menu_options,
                },
            )),
        )
        .await?;

    Ok(())
}

pub async fn get(ctx: &Context, interaction: &CommandInteraction, id: &str) -> Result<()> {
    let mut stream = SUPPORT_FAQ_CHANNEL_ID.messages_iter(ctx).boxed();

    while let Some(msg) = stream.try_next().await? {
        let support_id = msg
            .content
            .lines()
            .next()
            .ok_or(Error::EmptyMessage)?
            .trim();

        let title = &support_id[2..support_id.len() - 2];
        let description = msg.content.strip_prefix(support_id).unwrap();

        if support_id.contains(id) {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new()
                        .embed(CreateEmbed::new().title(title).description(description)),
                )
                .await?;
            return Ok(());
        }
    }

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Support message not found"),
        )
        .await?;
    Ok(())
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let command = &interaction.data.options()[0];

    let options = match &command.value {
        ResolvedValue::SubCommand(options) => options,
        _ => unreachable!("Subcommand is required"),
    };
    let options = parse_options(options);

    match command.name {
        "list" => {
            interaction.defer_ephemeral(ctx).await?;
            list(ctx, interaction).await?;
        }
        "get" => {
            interaction.defer(ctx).await?;

            let id = match options.get("id") {
                Some(ResolvedValue::String(id)) => *id,
                _ => unreachable!("ID is required"),
            };
            get(ctx, interaction, id).await?;
        }
        _ => unreachable!("Invalid subcommand"),
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("support")
        .description("Displays a support message")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "list",
            "Lists all support messages",
        ))
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "get",
                "Displays a support message",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "id",
                    "The ID of the support message",
                )
                .required(true),
            ),
        )
}
