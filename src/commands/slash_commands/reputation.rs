use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;

pub fn run<'a>(_interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    response.interaction_response_data(|message| message.embed(|e| {
        e.field("Popular", "❌ Bro\n✅ Trouble Maker\n❌ Boyfriend", true)
            .field("Loyal", "✅ Bro\n✅ Boyfriend\n❌ Trouble Maker", true)
            .field("Confident", "✅ Boyfriend\n✅ Trouble Maker\n❌ Bro", true)
    }));
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("reputation").description("View the secrets behind the reputation value")
}