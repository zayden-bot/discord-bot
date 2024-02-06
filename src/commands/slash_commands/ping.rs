use crate::utils::message_response;
use serenity::all::{CommandInteraction, Context, CreateCommand, Message};

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    message_response(&ctx, interaction, "Pong!").await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
