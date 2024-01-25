use crate::utils::respond_with_message;
use serenity::all::{CommandInteraction, Context, CreateCommand};

pub async fn run(ctx: Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return respond_with_message(
                &ctx,
                interaction,
                "This command can only be used in a server",
            )
            .await
        }
    };

    let partial_guild = ctx.http.get_guild_with_counts(guild_id).await?;

    respond_with_message(
        &ctx,
        interaction,
        &format!(
            "There are **{}** members in this server",
            partial_guild.approximate_member_count.unwrap_or_default()
        ),
    )
    .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("member_count").description("View the total member count")
}
