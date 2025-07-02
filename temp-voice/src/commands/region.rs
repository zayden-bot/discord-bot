use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, Context, EditChannel, EditInteractionResponse, ResolvedValue,
};

use crate::error::PermissionError;
use crate::{Error, VoiceChannelRow};

pub async fn region(
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

    let region = match options.remove("region") {
        Some(ResolvedValue::String(region)) => Some(region.to_string()),
        _ => None,
    };

    channel_id
        .edit(ctx, EditChannel::new().voice_region(region))
        .await
        .unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel region updated."),
        )
        .await
        .unwrap();

    Ok(())
}
