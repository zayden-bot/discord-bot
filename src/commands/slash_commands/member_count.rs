use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub fn run<'a>(ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            response.interaction_response_data(|message| message.content("This command can only be used in a server"));
            return response;
        },
    };

    let guild = match guild_id.to_guild_cached(ctx) {
        Some(guild) => guild,
        None => {
            response.interaction_response_data(|message| message.content("Error retrieving guild"));
            return response;
        },
    };

    response.interaction_response_data(|message| message.content(format!("There are **{}** members in this server", guild.member_count)));
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("member_count").description("View the total member count")
}