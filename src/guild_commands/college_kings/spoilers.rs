use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, EditInteractionResponse, Mentionable, Ready,
    ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::guilds::ServersTable;
use crate::{Error, Result};

pub struct Spoilers;

#[async_trait]
impl SlashCommand<Error, Postgres> for Spoilers {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let row = ServersTable::get_row(pool, guild_id)
            .await
            .unwrap()
            .unwrap();
        let support_channel_id = row.get_support_channel_id()?;
        let spoiler_channel_id = row.get_spoiler_channel_id()?;

        let content = format!(
            "Spoilers are defined as any content that has not been released on all supported platforms for at least 2 weeks.
            Please keep all conversations about spoilers to {}.
            If you have any bugs or questions please post them in {}",
            support_channel_id.mention(), spoiler_channel_id.mention());

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content(content))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("spoilers").description("Disclaimer about spoilers");

        Ok(command)
    }
}
