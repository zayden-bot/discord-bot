use crate::utils::send_embed;
use crate::Result;
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed, CreateMessage};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    send_embed(
        ctx,
        interaction,
        CreateMessage::new().embed(
            CreateEmbed::new()
                .field("Popular", "✅ Bro\n✅ Trouble Maker\n❌ Boyfriend", true)
                .field("Loyal", "✅ Bro\n✅ Boyfriend\n❌ Trouble Maker", true)
                .field("Confident", "✅ Boyfriend\n✅ Trouble Maker\n❌ Bro", true),
        ),
    )
    .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("reputation").description("View the secrets behind the reputation value")
}
