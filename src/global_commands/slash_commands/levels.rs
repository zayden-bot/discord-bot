use serenity::all::{
    CommandInteraction, Context, CreateButton, CreateCommand, CreateEmbed, CreateEmbedFooter,
    EditInteractionResponse, Ready,
};

use crate::sqlx_lib::user_levels::get_users;
use crate::Result;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(&ctx).await?;

    let page_number = 1;

    let fields = get_users(ctx, page_number, 10)
        .await?
        .into_iter()
        .map(|level_data| {
            (
                level_data.user.name,
                format!(
                    "Messages: {} | Total XP: {} | Level: {}",
                    level_data.message_count, level_data.xp, level_data.level
                ),
                false,
            )
        });

    let embed = CreateEmbed::new()
        .title("Leaderboard")
        .fields(fields)
        .footer(CreateEmbedFooter::new(format!("Page {}", page_number)));

    interaction
        .edit_response(
            &ctx,
            EditInteractionResponse::new()
                .embed(embed)
                .button(CreateButton::new("levels_previous").label("<"))
                .button(CreateButton::new("levels_user").emoji('🎯'))
                .button(CreateButton::new("levels_next").label(">")),
        )
        .await?;

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("levels").description("Get the leaderboard");

    Ok(command)
}
