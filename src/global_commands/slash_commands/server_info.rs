use crate::utils::send_embed;
use crate::{Error, Result};
use serenity::all::{
    ChannelType, Command, CommandInteraction, Context, CreateCommand, CreateEmbed,
    CreateEmbedAuthor, CreateMessage,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NoGuild)?;
    let partial_guild = guild_id.to_partial_guild_with_counts(&ctx).await?;

    let mut category_channel_count = 0;
    let mut text_channel_count = 0;
    let mut voice_channel_count = 0;

    guild_id
        .channels(&ctx)
        .await?
        .values()
        .for_each(|channel| match channel.kind {
            ChannelType::Category => category_channel_count += 1,
            ChannelType::Text => text_channel_count += 1,
            ChannelType::Voice => voice_channel_count += 1,
            _ => (),
        });

    send_embed(
        ctx,
        interaction,
        CreateMessage::new().embed(
            CreateEmbed::new()
                .author(
                    CreateEmbedAuthor::new(&partial_guild.name)
                        .icon_url(partial_guild.icon_url().unwrap_or_default()),
                )
                .field("Owner", format!("<@{}>", partial_guild.owner_id), true)
                .field(
                    "Channel Categories",
                    category_channel_count.to_string(),
                    true,
                )
                .field("Text Channels", text_channel_count.to_string(), true)
                .field("Voice Channels", voice_channel_count.to_string(), true)
                .field(
                    "Members",
                    partial_guild
                        .approximate_member_count
                        .unwrap_or_default()
                        .to_string(),
                    true,
                )
                .field("Roles", partial_guild.roles.len().to_string(), true),
        ),
    )
    .await?;

    Ok(())
}
pub async fn register(ctx: &Context) -> Result<()> {
    Command::create_global_command(
        ctx,
        CreateCommand::new("server_info").description("Get information about the server"),
    )
    .await?;

    Ok(())
}
