use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, Context, EditInteractionResponse, PermissionOverwrite,
    PermissionOverwriteType, Permissions, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::error::PermissionError;
use crate::{Error, Result, VoiceChannelManager, VoiceChannelRow};

pub async fn transfer<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    mut options: HashMap<&str, ResolvedValue<'_>>,
    channel_id: ChannelId,
    mut row: VoiceChannelRow,
) -> Result<()> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    if !row.is_owner(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    let user = match options.remove("user") {
        Some(ResolvedValue::User(user, _)) => user,
        _ => unreachable!("User option is required"),
    };

    row.set_owner(user.id);
    row.save::<Db, Manager>(pool).await?;

    channel_id
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::all(),
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user.id),
            },
        )
        .await
        .unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Transferred channel."),
        )
        .await
        .unwrap();

    Ok(())
}
