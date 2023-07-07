use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::sqlx_lib::get_support_channel_ids;

pub async fn run<'a>(_ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            response.interaction_response_data(|message| message.content("This command can only be used in a server"));
            return response;
        },
    };

    let support_thread_ids = match get_support_channel_ids(guild_id.0 as i64).await {
        Ok(support_thread_ids) => support_thread_ids,
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error retrieving support channel"));
            return response;
        },
    };

    let support_thread_id = match support_thread_ids.first() {
        Some(support_thread_id) => support_thread_id,
        None => {
            response.interaction_response_data(|message| message.content("Error retrieving support channel"));
            return response;
        },
    };

    response.interaction_response_data(|message| message.content(format!("We do our best to retain save integrity with every update however due to the dynamic nature of game development saves might break. If you experience a save problem please let us know in <#{}>", support_thread_id)));
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("saves").description("Get saves disclaimer")
}