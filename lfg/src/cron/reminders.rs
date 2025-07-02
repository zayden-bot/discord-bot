use chrono::{Datelike, Duration, Timelike};
use futures::future;
use serenity::all::{ChannelId, Colour, Context, CreateEmbed, CreateMessage, Mentionable};
use sqlx::{Database, Pool};
use zayden_core::{CronJob, cron::CronJobs};

use crate::{Join, PostManager, PostRow};

pub async fn create_reminders<Db: Database, Manager: PostManager<Db>>(
    ctx: &Context,
    row: &PostRow,
) {
    let post_id = row.channel();

    let week = row.start_time - Duration::days(7);
    let day = row.start_time - Duration::hours(24);
    let mins_30 = row.start_time - Duration::minutes(30);

    let week_job = CronJob::<Db>::new(&format!(
        "0 {} {} {} {} * {}",
        week.minute(),
        week.hour(),
        week.day(),
        week.month(),
        week.year()
    ))
    .set_action(move |ctx, pool| async move {
        reminder::<Db, Manager>(ctx, pool, post_id).await;
    });

    let day_job = CronJob::<Db>::new(&format!(
        "0 {} {} {} {} * {}",
        day.minute(),
        day.hour(),
        day.day(),
        day.month(),
        day.year()
    ))
    .set_action(move |ctx, pool| async move {
        reminder::<Db, Manager>(ctx, pool, post_id).await;
    });

    let mins_30_job = CronJob::<Db>::new(&format!(
        "0 {} {} {} {} * {}",
        mins_30.minute(),
        mins_30.hour(),
        mins_30.day(),
        mins_30.month(),
        mins_30.year()
    ))
    .set_action(move |ctx, pool| async move {
        reminder::<Db, Manager>(ctx, pool, post_id).await;
    });

    let mut data = ctx.data.write().await;
    let jobs = data.entry::<CronJobs<Db>>().or_insert(Vec::new());

    jobs.extend([week_job, day_job, mins_30_job]);
}

async fn reminder<Db: Database, Manager: PostManager<Db>>(
    ctx: Context,
    pool: Pool<Db>,
    id: ChannelId,
) {
    let post = match Manager::row(&pool, id).await {
        Ok(post) => post,
        Err(sqlx::Error::RowNotFound) => {
            println!("Post for '{}' not found", id);
            return;
        }
        Err(e) => panic!("{e:?}"),
    };

    let timestamp = post.start_time.timestamp();

    let embed = CreateEmbed::new()
        .title(format!("{} - <t:{timestamp}>", &post.activity))
        .colour(Colour::BLUE)
        .description(format!(
            "Starting <t:{timestamp}:R>\nThread: {}",
            post.channel().mention()
        ))
        .field(
            "Joined",
            post.fireteam()
                .map(|user| user.mention().to_string())
                .collect::<Vec<_>>()
                .join(" | "),
            false,
        );

    let iter = post
        .fireteam()
        .map(|user| user.dm(&ctx, CreateMessage::new().embed(embed.clone())));

    future::join_all(iter).await;
}
