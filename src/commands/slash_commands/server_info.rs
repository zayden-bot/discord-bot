use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{Channel, ChannelType};
use serenity::prelude::Context;
use crate::utils::{respond_with_embed, respond_with_message};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let guild = match guild_id.to_guild_cached(ctx) {
        Some(guild) => guild,
        None => return respond_with_message(ctx, interaction, "Error retrieving server information").await,
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

    respond_with_embed(ctx, interaction, |e| {
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
    }).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("server_info").description("Get information about the server")
}