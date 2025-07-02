use chrono::{Duration, Utc};
use serenity::all::{
    Context, DiscordJsonError, EditThread, ErrorResponse, Guild, HttpError, PartialGuildChannel,
};
use sqlx::{Database, Pool};

use crate::{GuildManager, PostManager, actions, cron::create_reminders, templates::TemplateInfo};

pub async fn thread_delete<Db: Database, Manager: PostManager<Db>>(
    ctx: &Context,
    thread: &PartialGuildChannel,
    pool: &Pool<Db>,
) {
    if Manager::exists(pool, thread.id).await.unwrap() {
        actions::delete::<Db, Manager>(ctx, thread.id, pool)
            .await
            .unwrap();
    }
}

pub async fn guild_create<
    Db: Database,
    GuildHandler: GuildManager<Db>,
    PostHandler: PostManager<Db>,
>(
    ctx: &Context,
    guild: &Guild,
    pool: &Pool<Db>,
) {
    let Ok(Some(guild_row)) = GuildHandler::row(pool, guild.id).await else {
        return;
    };

    let lfg_channel = guild_row.channel_id();

    let archived_threads = lfg_channel
        .get_archived_public_threads(&ctx, None, Some(100))
        .await
        .unwrap();

    let threads = guild
        .threads
        .iter()
        .filter(|thread| thread.parent_id.is_some_and(|id| id == lfg_channel))
        .chain(archived_threads.threads.iter())
        .cloned();

    let now = Utc::now();
    let week_ago = now - Duration::days(7);
    let month_ago = now - Duration::days(30);

    for mut thread in threads {
        let created_at = *thread.last_message_id.unwrap().created_at();

        if created_at < month_ago {
            thread.delete(ctx).await.unwrap();
        }

        if created_at < week_ago {
            match thread
                .edit_thread(ctx, EditThread::new().archived(true))
                .await
            {
                Ok(_)
                // Unknown Channel
                | Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                    error: DiscordJsonError { code: 10003, .. },
                    ..
                }))) => {}
                Err(e) => panic!("{e:?}"),
            }
        }

        let post = match PostHandler::row(pool, thread.id).await {
            Ok(post) => post,
            Err(_) => continue,
        };

        if post.start_time > now {
            create_reminders::<Db, PostHandler>(ctx, &post).await;
        }

        if post.start_time < now {
            if let (Some(channel), Some(message)) = (post.alt_channel(), post.alt_message()) {
                match channel.delete_message(ctx, message).await {
                    Ok(_)
                    // Unknown Message
                    | Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                        error: DiscordJsonError { code: 10008, .. },
                        ..
                    }))) => {}
                    Err(e) => panic!("{e:?}"),
                };
            }
        }

        if post.start_time + Duration::hours(2) < now {
            post.channel()
                .edit_thread(ctx, EditThread::new().archived(true))
                .await
                .unwrap();
        }

        /*
            for post in posts {
                let thread = post.channel();

                if !threads.contain(thread) {
                    actions::delete::<Db, PostHandler>(ctx, thread, pool)
                            .await
                            .unwrap();
                }
            }
        */
    }
}
