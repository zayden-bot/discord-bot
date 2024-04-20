use crate::utils::embed_response;
use crate::Result;
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    embed_response(
        ctx,
        interaction,
        CreateEmbed::new()
            .field("Popular", "✅ Bro\n✅ Trouble Maker\n❌ Boyfriend", true)
            .field("Loyal", "✅ Bro\n✅ Boyfriend\n❌ Trouble Maker", true)
            .field("Confident", "✅ Boyfriend\n✅ Trouble Maker\n❌ Bro", true),
    )
    .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("reputation").description("View the secrets behind the reputation value")
}
