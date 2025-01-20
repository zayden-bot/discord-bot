use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, EditInteractionResponse, Ready, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct MemberCount;

#[async_trait]
impl SlashCommand<Error, Postgres> for MemberCount {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let partial_guild = guild_id.to_partial_guild_with_counts(&ctx).await.unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content(format!(
                    "There are **{}** members in this server",
                    partial_guild.approximate_member_count.unwrap_or_default()
                )),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("member_count").description("View the total member count");

        Ok(command)
    }
}
