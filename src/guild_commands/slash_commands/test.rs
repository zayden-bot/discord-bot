use serenity::all::{CommandInteraction, Context, CreateCommand, GuildId, Permissions};

use crate::{utils::message_response, Result, COLLEGE_KINGS_GUILD_ID};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    message_response(ctx, interaction, "Sent test command").await?;
    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    GuildId::new(COLLEGE_KINGS_GUILD_ID)
        .create_command(
            ctx,
            CreateCommand::new("test")
                .description("Test command")
                .default_member_permissions(Permissions::ADMINISTRATOR),
        )
        .await?;

    Ok(())
}
