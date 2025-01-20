use async_trait::async_trait;
use serenity::all::{
    ChannelType, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateEmbedAuthor,
    EditInteractionResponse, Ready, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct ServerInfo;

#[async_trait]
impl SlashCommand<Error, Postgres> for ServerInfo {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;
        let partial_guild = guild_id.to_partial_guild_with_counts(&ctx).await.unwrap();

        let (category_count, text_count, voice_count) =
            guild_id.channels(&ctx).await.unwrap().values().fold(
                (0, 0, 0),
                |(mut cat, mut text, mut voice), channel| {
                    match channel.kind {
                        ChannelType::Category => cat += 1,
                        ChannelType::Text => text += 1,
                        ChannelType::Voice => voice += 1,
                        _ => (),
                    }

                    (cat, text, voice)
                },
            );

        let embed = CreateEmbed::new()
            .author(
                CreateEmbedAuthor::new(&partial_guild.name)
                    .icon_url(partial_guild.icon_url().unwrap_or_default()),
            )
            .field("Owner", format!("<@{}>", partial_guild.owner_id), true)
            .field("Channel Categories", category_count.to_string(), true)
            .field("Text Channels", text_count.to_string(), true)
            .field("Voice Channels", voice_count.to_string(), true)
            .field(
                "Members",
                partial_guild
                    .approximate_member_count
                    .unwrap_or_default()
                    .to_string(),
                true,
            )
            .field("Roles", partial_guild.roles.len().to_string(), true);

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command =
            CreateCommand::new("server_info").description("Get information about the server");

        Ok(command)
    }
}
