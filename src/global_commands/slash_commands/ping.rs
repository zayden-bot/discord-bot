use crate::utils::message_response;
use crate::Result;
use serenity::all::{CommandInteraction, Context, CreateCommand};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    message_response(ctx, interaction, "Pong!").await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
