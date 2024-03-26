use crate::utils::message_response;
use crate::Result;
use serenity::all::{Command, CommandInteraction, Context, CreateCommand};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    message_response(ctx, interaction, "Pong!").await?;

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    Command::create_global_command(
        ctx,
        CreateCommand::new("ping").description("A ping command"),
    )
    .await?;

    Ok(())
}
