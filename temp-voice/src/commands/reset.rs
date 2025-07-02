use serenity::all::{
    ChannelId, CommandInteraction, Context, EditChannel, EditInteractionResponse, GuildId,
    PermissionOverwriteType,
};
use sqlx::{Database, Pool};

use crate::error::PermissionError;
use crate::{Error, Result, VoiceChannelManager, VoiceChannelRow};

pub async fn reset<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    guild_id: GuildId,
    channel_id: ChannelId,
    mut row: VoiceChannelRow,
) -> Result<()> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    if !row.is_owner(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    row.reset();
    row.save::<Db, Manager>(pool).await?;

    let channel = guild_id
        .channels(ctx)
        .await
        .unwrap()
        .remove(&channel_id)
        .ok_or(Error::ChannelNotFound(channel_id))?;

    let everyone_permissions = channel
        .permission_overwrites
        .iter()
        .find(|perm| perm.kind == PermissionOverwriteType::Role(guild_id.everyone_role()))
        .expect("Expected everyone role in channel permissions");

    channel_id
        .edit(
            ctx,
            EditChannel::new()
                .name(format!("{}'s Channel", interaction.user.display_name()))
                .user_limit(0)
                .permissions(vec![everyone_permissions.clone()]),
        )
        .await
        .unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Reset channel."),
        )
        .await
        .unwrap();

    Ok(())
}
