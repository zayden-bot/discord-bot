use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, Context, EditInteractionResponse, PermissionOverwriteType,
    ResolvedValue,
};

use crate::error::PermissionError;
use crate::{Error, VoiceChannelRow};

pub async fn unblock(
    ctx: &Context,
    interaction: &CommandInteraction,
    mut options: HashMap<&str, ResolvedValue<'_>>,
    channel_id: ChannelId,
    row: &VoiceChannelRow,
) -> Result<(), Error> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    if !row.is_trusted(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotTrusted));
    }

    let user = match options.remove("user") {
        Some(ResolvedValue::User(user, _member)) => user,
        _ => unreachable!("User option is required"),
    };

    channel_id
        .delete_permission(ctx, PermissionOverwriteType::Member(user.id))
        .await
        .unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Removed user from blocked."),
        )
        .await
        .unwrap();

    Ok(())
}
