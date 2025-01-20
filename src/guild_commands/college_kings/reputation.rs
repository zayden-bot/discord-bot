use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, Ready, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct Reputation;

#[async_trait]
impl SlashCommand<Error, Postgres> for Reputation {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        let embed = CreateEmbed::new()
            .field("Popular", "✅ Bro\n✅ Trouble Maker\n❌ Boyfriend", true)
            .field("Loyal", "✅ Bro\n✅ Boyfriend\n❌ Trouble Maker", true)
            .field("Confident", "✅ Boyfriend\n✅ Trouble Maker\n❌ Bro", true);

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().embed(embed),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("reputation")
            .description("View the secrets behind the reputation value");

        Ok(command)
    }
}
