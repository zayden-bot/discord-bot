pub mod availability_check;

use async_trait::async_trait;
use availability_check::AvailabilityCheck;
use cron::Schedule;
use serenity::all::Context;
use std::time::Duration;
use tokio::time::sleep;

use crate::modules::patreon::cache::PatreonCache;
use crate::Result;

#[async_trait]
pub trait CronJob {
    fn schedule(&self) -> Schedule;
    async fn action(&self, ctx: &Context) -> Result<()>;
}

pub async fn start_cron_jobs(ctx: Context) {
    if let Err(e) = _start_cron_jobs(ctx).await {
        eprintln!("Error starting cron jobs: {:?}", e);
    }
}

async fn _start_cron_jobs(ctx: Context) -> Result<()> {
    let mut jobs: Vec<Box<dyn CronJob + Send>> =
        vec![Box::new(AvailabilityCheck), Box::new(PatreonCache)];

    jobs.sort_by(|a, b| {
        let a = a
            .schedule()
            .upcoming(chrono::Utc)
            .next()
            .unwrap_or_default();
        let b = b
            .schedule()
            .upcoming(chrono::Utc)
            .next()
            .unwrap_or_default();
        a.cmp(&b)
    });

    loop {
        let job = jobs.remove(0);

        if let Some(when) = job.schedule().upcoming(chrono::Utc).next() {
            let now = chrono::Utc::now();
            let delta = when - now;
            let duration = Duration::from_secs(delta.num_seconds() as u64);
            println!("Next job: {:?}", when);
            sleep(duration).await;

            job.action(&ctx).await?;
            sleep(Duration::from_secs(60)).await;
        }

        jobs.push(job);
    }
}
