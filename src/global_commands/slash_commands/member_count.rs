use crate::{utils::message_response, Error, Result};
use serenity::all::{CommandInteraction, Context, CreateCommand, Ready};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NotInGuild)?;

    let partial_guild = guild_id.to_partial_guild_with_counts(&ctx).await.unwrap();

    message_response(
        ctx,
        interaction,
        &format!(
            "There are **{}** members in this server",
            partial_guild.approximate_member_count.unwrap_or_default()
        ),
    )
    .await
    .unwrap();

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("member_count").description("View the total member count");

    Ok(command)
}
