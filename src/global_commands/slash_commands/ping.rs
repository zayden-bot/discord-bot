use crate::utils::message_response;
use crate::Result;
use serenity::all::{CommandInteraction, Context, CreateCommand, Ready};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    message_response(ctx, interaction, "Pong!").await?;

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("ping").description("A ping command");

    Ok(command)
}
