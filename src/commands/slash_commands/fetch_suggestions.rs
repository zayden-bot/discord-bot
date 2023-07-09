use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

const SUGGESTION_CHANNEL_ID: u64 = 1068790374996377671;

pub async fn run<'a>(ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            response.interaction_response_data(|message| message.content("This command can only be used in a server"));
            return response;
        }
    };

    let guild_channels = match guild_id.channels(&ctx).await {
        Ok(guild_channels) => guild_channels,
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error getting guild channels"));
            return response;
        }
    };

    let suggestion_channel_id = match guild_channels.values().find(|channel| channel.id == SUGGESTION_CHANNEL_ID) {
        Some(suggestion_channel) => suggestion_channel,
        None => {
            response.interaction_response_data(|message| message.content("Error getting suggestion channel"));
            return response;
        }
    };

    let x = suggestion_channel_id.threads(&ctx).unwrap();

    response.interaction_response_data(|message| message.content("Hey, I'm alive!"));
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("fetch_suggestions").description("Fetch suggestions from the suggestion channel")
}