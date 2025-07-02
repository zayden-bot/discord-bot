use serenity::all::{ChannelId, Context, DiscordJsonError, ErrorResponse, HttpError};
use sqlx::{Database, Pool};

use crate::{PostManager, Result, templates::TemplateInfo};

pub async fn delete<Db: Database, Manager: PostManager<Db>>(
    ctx: &Context,
    channel: ChannelId,
    pool: &Pool<Db>,
) -> Result<()> {
    let Ok(post) = Manager::row(pool, channel).await else {
        return Ok(());
    };

    match post.channel().delete(ctx).await {
        Ok(_)
        // Unknown Channel
        | Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 10003, .. },
            ..
        }))) => {}
        Err(e) => panic!("{e:?}"),
    }

    if let (Some(channel), Some(message)) = (post.alt_channel(), post.alt_message()) {
        match channel.delete_message(ctx, message).await {
            Ok(_)
            // Unknown Message
            | Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 10008, .. },
                ..
            }))) => {}
            Err(e) => panic!("{e:?}"),
        }
    }

    Manager::delete(pool, channel).await.unwrap();

    Ok(())
}
