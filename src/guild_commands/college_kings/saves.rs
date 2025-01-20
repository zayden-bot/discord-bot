use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, EditInteractionResponse, Ready, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::guilds::ServersTable;
use crate::{Error, Result};

pub struct Saves;

#[async_trait]
impl SlashCommand<Error, Postgres> for Saves {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(&ctx).await.unwrap();

        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let support_thread_id = ServersTable::get_row(pool, guild_id)
            .await
            .unwrap()
            .unwrap()
            .get_support_channel_id()
            .unwrap();

        interaction.edit_response(ctx, EditInteractionResponse::new().content(format!("We do our best to retain save integrity with every update however due to the dynamic nature of game development saves might break. If you experience a save problem please let us know in <#{}>", support_thread_id))).await.unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("saves").description("Get saves disclaimer");

        Ok(command)
    }
}
