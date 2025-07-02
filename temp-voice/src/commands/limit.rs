use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, Context, EditChannel, EditInteractionResponse, ResolvedValue,
};

use crate::error::PermissionError;
use crate::{Error, VoiceChannelRow};

pub async fn limit(
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

    let limit = match options.remove("user_limit") {
        Some(ResolvedValue::Integer(limit)) => limit.clamp(0, 99) as u32,
        _ => 0,
    };

    channel_id
        .edit(ctx, EditChannel::new().user_limit(limit))
        .await
        .unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content(format!("User limit set to {}", limit)),
        )
        .await
        .unwrap();

    Ok(())
}
