use crate::utils::respond_with_embed;
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    respond_with_embed(
        ctx,
        interaction,
        CreateEmbed::new()
            .field("Popular", "✅ Bro\n✅ Trouble Maker\n❌ Boyfriend", true)
            .field("Loyal", "✅ Bro\n✅ Boyfriend\n❌ Trouble Maker", true)
            .field("Confident", "✅ Boyfriend\n✅ Trouble Maker\n❌ Bro", true),
    )
    .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("reputation").description("View the secrets behind the reputation value")
}
