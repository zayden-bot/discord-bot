use serenity::all::{
    ChannelType, Context, CreateChannel, CreateMessage, DiscordJsonError, ErrorResponse, HttpError,
    PermissionOverwrite, PermissionOverwriteType, Permissions, VoiceState,
};
use sqlx::{Database, Pool};

use crate::{
    Result, TempVoiceGuildManager, VoiceChannelManager, VoiceChannelRow,
    delete_voice_channel_if_inactive,
};

pub async fn channel_creator<
    Db: Database,
    GuildManager: TempVoiceGuildManager<Db>,
    ChannelManager: VoiceChannelManager<Db>,
>(
    ctx: &Context,
    pool: &Pool<Db>,
    new: &VoiceState,
) -> Result<()> {
    let guild_id = new
        .guild_id
        .expect("Should be in a guild as voice channels are guild only");

    let Ok(Some(creator_channel)) = GuildManager::get_creator_channel(pool, guild_id).await else {
        return Ok(());
    };

    let creator_channel_id = match new.channel_id {
        Some(channel) if channel == creator_channel => channel,
        _ => return Ok(()),
    };

    let creator_category = creator_channel_id
        .to_channel(ctx)
        .await
        .unwrap()
        .guild()
        .expect("Should be in a guild")
        .parent_id
        .expect("Should be in a category");

    let member = new.member.as_ref().expect("Should be in a guild");

    let perms = vec![PermissionOverwrite {
        allow: Permissions::all(),
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(member.user.id),
    }];

    let vc_builder = CreateChannel::new(format!("{}'s Channel", member.display_name()))
        .kind(ChannelType::Voice)
        .category(creator_category)
        .permissions(perms);

    let vc = match guild_id.create_channel(ctx, vc_builder).await {
        // Missing Permission
        Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 50013, .. },
            ..
        }))) => return Ok(()),
        r => r?,
    };

    match guild_id.move_member(ctx, member.user.id, vc.id).await {
        // Target user is not connected to voice.
        Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 40032, .. },
            ..
        }))) => {
            member
                .user
                .direct_message(
                    ctx,
                    CreateMessage::new()
                        .content("Voice channel created. You have 1 minute to join."),
                )
                .await
                .unwrap();

            if delete_voice_channel_if_inactive(ctx, guild_id, member.user.id, &vc).await {
                return Ok(());
            }
        }
        result => {
            result.unwrap();
        }
    };

    let row = VoiceChannelRow::new(vc.id, new.user_id);
    row.save::<Db, ChannelManager>(pool).await?;

    Ok(())
}
