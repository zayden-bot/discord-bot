use serenity::all::{Context, DiscordJsonError, ErrorResponse, HttpError};
use sqlx::{Database, Pool};

use crate::{CachedState, Result, TempVoiceGuildManager, VoiceChannelManager, VoiceStateCache};

pub async fn channel_deleter<
    Db: Database,
    GuildManager: TempVoiceGuildManager<Db>,
    ChannelManager: VoiceChannelManager<Db>,
>(
    ctx: &Context,
    pool: &Pool<Db>,
    old: Option<&CachedState>,
) -> Result<()> {
    let old = match old {
        Some(old) => old,
        None => return Ok(()),
    };

    let Ok(guild_data) = GuildManager::get(pool, old.guild_id).await else {
        return Ok(());
    };

    let channel_id = match old.channel_id {
        Some(channel_id) if channel_id != guild_data.creator_channel() => channel_id,
        _ => return Ok(()),
    };

    let row = match ChannelManager::get(pool, channel_id).await.unwrap() {
        Some(row) => row,
        None => return Ok(()),
    };

    if row.is_persistent() {
        return Ok(());
    }

    let channel = match channel_id.to_channel(ctx).await {
        // Unknown channel
        Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 10003, .. },
            ..
        })))
        // Missing access
        | Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 50001, .. },
            ..
        }))) => {
            return Ok(());
        }
        r => r?,
    };
    let category = guild_data.category();

    if channel
        .guild()
        .expect("Should be in a guild")
        .parent_id
        .expect("Should be in a category")
        != category
    {
        return Ok(());
    }

    let users = {
        let data = ctx.data.read().await;
        let cache = data
            .get::<VoiceStateCache>()
            .expect("Expected VoiceStateCache in TypeMap");

        cache
            .values()
            .filter(|id| id.channel_id == Some(channel_id))
            .count()
    };

    if users == 0 {
        row.delete::<Db, ChannelManager>(pool).await?;

        match channel_id.delete(ctx).await {
            Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 10003, .. },
                ..
            }))) => {
                // Channel already deleted, ignore this error
            }
            result => {
                result.unwrap();
            }
        };
    }

    Ok(())
}
