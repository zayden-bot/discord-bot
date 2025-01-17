use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditInteractionResponse, Ready,
    ResolvedOption, ResolvedValue,
};

use crate::{guilds::college_kings::FAQ_CHANNEL_ID, Error, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    let options = interaction.data.options();

    let select_menu_id = match options.first() {
        Some(ResolvedOption {
            value: ResolvedValue::Boolean(false),
            ..
        }) => "faq",
        _ => "faq_ephemeral",
    };

    let mut menu_options: Vec<CreateSelectMenuOption> = FAQ_CHANNEL_ID
        .messages_iter(ctx)
        .enumerate()
        .then(|(index, msg_result)| async move {
            let msg = msg_result.unwrap();
            let id = msg.content.lines().next().unwrap().trim();

            Ok::<CreateSelectMenuOption, Error>(CreateSelectMenuOption::new(
                id[2..id.len() - 2].to_string(),
                index.to_string(),
            ))
        })
        .try_collect()
        .await?;
    menu_options.pop();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::default().select_menu(CreateSelectMenu::new(
                select_menu_id,
                CreateSelectMenuKind::String {
                    options: menu_options,
                },
            )),
        )
        .await
        .unwrap();

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("faq")
        .description("Displays a FAQ message")
        .add_option(CreateCommandOption::new(
            CommandOptionType::Boolean,
            "ephemeral",
            "Whether the response should be ephemeral | Default: true",
        ));

    Ok(command)
}
