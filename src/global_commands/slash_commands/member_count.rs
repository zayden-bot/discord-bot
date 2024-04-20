use crate::{utils::message_response, Error, Result};
use serenity::all::{CommandInteraction, Context, CreateCommand};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let partial_guild = guild_id.to_partial_guild_with_counts(&ctx).await?;

    message_response(
        ctx,
        interaction,
        &format!(
            "There are **{}** members in this server",
            partial_guild.approximate_member_count.unwrap_or_default()
        ),
    )
    .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("member_count").description("View the total member count")
}
