use crate::COLLEGE_KINGS_GUILD_ID;

use serenity::{
    all::{
        ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand,
        CreateCommandOption, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
        EditInteractionResponse, ResolvedOption, ResolvedValue,
    },
    futures::StreamExt,
};

use crate::Result;

pub const FAQ_CHANNEL_ID: ChannelId = ChannelId::new(1196346920059289690);

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer_ephemeral(ctx).await?;

    let options = interaction.data.options();

    let select_menu_id = match options.first() {
        Some(ResolvedOption {
            value: ResolvedValue::Boolean(false),
            ..
        }) => "faq",
        _ => "faq_ephemeral",
    };

    let mut ids = FAQ_CHANNEL_ID
        .messages_iter(ctx)
        .filter_map(|msg_result| async {
            match msg_result {
                Ok(msg) => Some(msg.content.lines().next()?.trim().to_string()),
                Err(_) => None,
            }
        })
        .collect::<Vec<_>>()
        .await;
    ids.pop();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::default().select_menu(CreateSelectMenu::new(
                select_menu_id,
                CreateSelectMenuKind::String {
                    options: ids
                        .into_iter()
                        .map(|id| CreateSelectMenuOption::new(&id[2..id.len() - 2], &id))
                        .collect(),
                },
            )),
        )
        .await?;

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    COLLEGE_KINGS_GUILD_ID
        .create_command(
            ctx,
            CreateCommand::new("faq")
                .description("Displays a FAQ message")
                .add_option(CreateCommandOption::new(
                    CommandOptionType::Boolean,
                    "ephemeral",
                    "Whether the response should be ephemeral",
                )),
        )
        .await?;

    Ok(())
}
