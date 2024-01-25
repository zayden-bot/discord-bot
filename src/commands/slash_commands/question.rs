use crate::utils::respond_with_ephemeral_message;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

const QUESTION_CHANNEL_ID: u64 = 829463308629180447;

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    respond_with_ephemeral_message(
        ctx,
        interaction,
        "This command is deprecated. Please use <#1196200376404291654> to post questions.",
    )
    .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("question").description("Ask a question")
}
