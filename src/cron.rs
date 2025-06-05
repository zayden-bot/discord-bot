use chrono::{DateTime, Utc};
use futures::future;
use serenity::all::Context;
use sqlx::{PgPool, Postgres};
use std::cmp::Ordering;
use std::time::Duration;
use tokio::time::sleep;
use zayden_core::{ActionFn, CronJob};

use crate::Result;

pub async fn start_cron_jobs(ctx: Context, pool: PgPool, jobs: Vec<CronJob<Postgres>>) {
    if let Err(e) = _start_cron_jobs(ctx, pool, jobs).await {
        eprintln!("Error starting cron jobs: {:?}", e);
    }
}

async fn _start_cron_jobs(ctx: Context, pool: PgPool, jobs: Vec<CronJob<Postgres>>) -> Result<()> {
    loop {
        let mut pending_jobs: Vec<(DateTime<Utc>, &ActionFn<Postgres>)> = Vec::new();

        for job in &jobs {
            let job_run_time = job.schedule.upcoming(Utc).next();
            let earliest_time = pending_jobs.first().map(|(time, _)| time);

            match (job_run_time, earliest_time) {
                (Some(job_run_time), Some(earliest_time)) => {
                    match job_run_time.cmp(earliest_time) {
                        Ordering::Less => pending_jobs = vec![(job_run_time, &job.action_fn)],
                        Ordering::Equal => pending_jobs.push((job_run_time, &job.action_fn)),
                        Ordering::Greater => {}
                    }
                }
                (Some(job_run_time), None) => {
                    pending_jobs = vec![(job_run_time, &job.action_fn)];
                }
                (None, _) => {
                    unreachable!("Job must have a scheduled occurrence: {}", job.schedule);
                }
            }
        }

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
    }
}
