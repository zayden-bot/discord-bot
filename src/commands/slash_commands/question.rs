use serenity::all::Message;
use serenity::{all::CommandInteraction, builder::CreateCommand, client::Context};

use crate::utils::send_message;

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    send_message(
        &ctx,
        interaction,
        "This command is deprecated. Please use <#1196200376404291654> to post questions.",
    )
    .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("question").description("Ask a question")
}
