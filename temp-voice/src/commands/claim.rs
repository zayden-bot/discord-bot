use serenity::all::{ChannelId, EditInteractionResponse};
use serenity::all::{
    CommandInteraction, Context, PermissionOverwrite, PermissionOverwriteType, Permissions,
};
use sqlx::{Database, Pool};

use crate::{Error, VoiceChannelManager, VoiceChannelRow, VoiceStateCache};

pub async fn claim<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    channel_id: ChannelId,
    row: Option<VoiceChannelRow>,
) -> Result<(), Error> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    let mut row = match row {
        Some(row) => {
            if row.is_owner(interaction.user.id) {
                return Err(Error::UserIsOwner);
            }

            row
        }
        None => VoiceChannelRow::new(channel_id, interaction.user.id),
    };

    if !row.is_persistent() && is_claimable(ctx, &row).await {
        return Err(Error::OwnerInChannel);
    }

    row.set_owner(interaction.user.id);
    row.save::<Db, Manager>(pool).await?;

    channel_id
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::all(),
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(interaction.user.id),
            },
        )
        .await
        .unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Claimed channel."),
        )
        .await
        .unwrap();

    Ok(())
}

async fn is_claimable(ctx: &Context, channel_data: &VoiceChannelRow) -> bool {
    let data = ctx.data.read().await;

    let owner_state = {
        let cache = data
            .get::<VoiceStateCache>()
            .expect("Expected VoiceStateCache in TypeMap");

        cache.get(&channel_data.owner_id())
    };

    owner_state.and_then(|state| state.channel_id) == Some(channel_data.channel_id())
}
