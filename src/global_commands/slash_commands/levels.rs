use serenity::all::{
    CommandInteraction, Context, CreateButton, CreateCommand, CreateEmbed, EditInteractionResponse,
};
use serenity::all::{CreateEmbedFooter, UserId};

use crate::sqlx_lib::user_levels::get_users;
use crate::sqlx_lib::PostgresPool;
use crate::Result;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(&ctx).await?;

    let pool = {
        let data = ctx.data.read().await;
        data.get::<PostgresPool>()
            .expect("PostgresPool should exist in data.")
            .clone()
    };

    let page_number = 1;
    let users = get_users(&pool, page_number, 10).await?;

    let mut fields = Vec::new();
    for level_data in users {
        let user = UserId::new(level_data.id as u64).to_user(ctx).await?;

        fields.push((
            user.name,
            format!(
                "Messages: {} | Total XP: {} | Level: {}",
                level_data.message_count, level_data.xp, level_data.level
            ),
            false,
        ));
    }

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

pub fn register() -> CreateCommand {
    CreateCommand::new("levels").description("Get the leaderboard")
}
