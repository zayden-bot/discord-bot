use crate::sqlx_lib::{get_pool, get_spoiler_channel_ids, get_support_channel_ids};
use crate::utils::message_response;
use crate::{Error, Result};
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateCommand, GuildChannel, GuildId, Mentionable,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(ctx).await?;

    let guild_id = interaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let pool = get_pool(ctx).await?;

    let no_channel = ChannelId::new(1);

    let support_thread_ids = get_support_channel_ids(&pool, guild_id.get()).await?;
    let support_thread_id = support_thread_ids.first().unwrap_or(&no_channel);

    let spoiler_thread_ids = get_spoiler_channel_ids(&pool, guild_id.get()).await?;
    let spoiler_thread_id = match spoiler_thread_ids.first() {
        Some(id) => *id,
        None => get_spoiler_channel_by_name(ctx, guild_id).await?.id,
    };

    message_response(ctx, interaction, &format!(
"Spoilers are defined as any content that has not been released on all supported platforms for at least 2 weeks.
Please keep all conversations about spoilers to {}.
If you have any bugs or questions please post them in {}",
spoiler_thread_id.mention(), support_thread_id.mention())).await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("spoilers").description("Disclaimer about spoilers")
}

async fn get_spoiler_channel_by_name(ctx: &Context, guild_id: GuildId) -> Result<GuildChannel> {
    guild_id
        .channels(&ctx)
        .await?
        .into_values()
        .find(|c| c.name.contains("spoilers"))
        .ok_or_else(|| Error::NoChannel)
}
