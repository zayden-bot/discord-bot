use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, Context, EditInteractionResponse, GuildId, PermissionOverwrite,
    PermissionOverwriteType, Permissions, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::error::PermissionError;
use crate::{Error, VoiceChannelRow, VoiceChannelManager};

pub async fn block<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    mut options: HashMap<&str, ResolvedValue<'_>>,
    guild_id: GuildId,
    channel_id: ChannelId,
    mut row: VoiceChannelRow,
) -> Result<(), Error> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    if !row.is_trusted(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotTrusted));
    }

    let user = match options.remove("user") {
        Some(ResolvedValue::User(user, _)) => user,
        _ => unreachable!("User option is required"),
    };

    row.block(user.id);
    row.save::<Db, Manager>(pool).await?;

    channel_id
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::all(),
                kind: PermissionOverwriteType::Member(user.id),
            },
        )
        .await
        .unwrap();

    guild_id.disconnect_member(ctx, user.id).await.unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Set user to blocked."),
        )
        .await
        .unwrap();

    Ok(())
}
