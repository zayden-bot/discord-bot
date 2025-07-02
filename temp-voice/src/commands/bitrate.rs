use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, Context, EditChannel, EditInteractionResponse, ResolvedValue,
};

use crate::error::PermissionError;
use crate::{Error, VoiceChannelRow};

pub async fn bitrate(
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

    let kbps = match options.remove("kbps") {
        Some(ResolvedValue::Integer(kbps)) => kbps as u32,
        _ => unreachable!("Kbps option is required"),
    };

    channel_id
        .edit(ctx, EditChannel::new().bitrate(kbps * 1000))
        .await
        .unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel bitrate updated."),
        )
        .await
        .unwrap();

    Ok(())
}
