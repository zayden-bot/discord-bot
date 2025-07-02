use chrono::{DateTime, Utc};
use futures::future;
use serenity::all::Context;
use sqlx::{PgPool, Postgres};
use std::cmp::Ordering;
use std::time::Duration;
use tokio::time::sleep;
use zayden_core::{ActionFn, CronJobs};

use crate::Result;

pub async fn start_cron_jobs(ctx: Context, pool: PgPool) {
    if let Err(e) = _start_cron_jobs(ctx, pool).await {
        eprintln!("Error starting cron jobs: {:?}", e);
    }
}

async fn _start_cron_jobs(ctx: Context, pool: PgPool) -> Result<()> {
    loop {
        let pending_jobs = pending_jobs(&ctx).await;

        let sleep_duration = match pending_jobs.first() {
            Some((target_wakeup_time, _)) => {
                println!("Next Job: {:?}", target_wakeup_time);

                let now = Utc::now();
                if *target_wakeup_time > now {
                    (*target_wakeup_time - now)
                        .to_std()
                        .unwrap_or(Duration::ZERO)
                } else {
                    Duration::ZERO
                }
            }
            None => Duration::from_secs(60),
        };

        if sleep_duration > Duration::from_millis(50) {
            sleep(sleep_duration).await;
        }

        if !pending_jobs.is_empty() {
            let futures_iter = pending_jobs
                .into_iter()
                .map(|(_, action)| (action)(ctx.clone(), pool.clone()));

            future::join_all(futures_iter).await;
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn pending_jobs(ctx: &Context) -> Vec<(DateTime<Utc>, ActionFn<Postgres>)> {
    let mut pending_jobs: Vec<(DateTime<Utc>, ActionFn<Postgres>)> = Vec::new();
    let mut earliest_time = None;

    let mut data = ctx.data.write().await;
    let jobs = data
        .remove::<CronJobs<Postgres>>()
        .unwrap_or(Vec::new())
        .into_iter()
        .filter_map(|job| {
            job.schedule
                .upcoming(Utc)
                .next()
                .map(|run_time| (job, run_time))
        })
        .collect::<Vec<_>>();

    for (job, run_time) in &jobs {
        let run_time = *run_time;

        match earliest_time {
            Some(time) => match run_time.cmp(&time) {
                Ordering::Less => {
                    earliest_time = Some(run_time);
                    pending_jobs = vec![(run_time, job.action_fn.clone())]
                }
                Ordering::Equal => pending_jobs.push((run_time, job.action_fn.clone())),
                Ordering::Greater => {}
            },
            None => {
                earliest_time = Some(run_time);
                pending_jobs = vec![(run_time, job.action_fn.clone())];
            }
        }
    }

    data.insert::<CronJobs<Postgres>>(jobs.into_iter().map(|(job, _)| job).collect());

    pending_jobs
}
