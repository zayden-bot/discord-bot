use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{Channel, ChannelType};
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

    let mut category_channel_count = 0;
    let mut text_channel_count = 0;
    let mut voice_channel_count = 0;

    guild.channels.values().for_each(|channel| match channel {
        Channel::Guild(guild_channel) => match guild_channel.kind {
            ChannelType::Text => text_channel_count += 1,
            ChannelType::Voice => voice_channel_count += 1,
            _ => (),
        }
        Channel::Category(_) => {
            category_channel_count += 1;
        },
        _ => (),
    });

    response.interaction_response_data(|message| message.embed(|e| {
        e.author(|author| {
            author.name(&guild.name);
            author.icon_url(guild.icon_url().unwrap_or_default())
        })
        .field("Owner", format!("<@{}>", guild.owner_id), true)
        .field("Channel Categories", category_channel_count, true)
        .field("Text Channels", text_channel_count, true)
        .field("Voice Channels", voice_channel_count, true)
        .field("Members", guild.member_count, true)
        .field("Roles", guild.roles.len(), true)
    }));
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("server_info").description("Get information about the server")
}