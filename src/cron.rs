use cron::Schedule;
use serenity::all::Context;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

use crate::guild_commands::college_kings::availability_check::availability_check_message;
use crate::guilds::college_kings_team::TEAM_LEADS_CHANNEL_ID;
use crate::Result;

pub async fn start_cron_jobs(ctx: Context) {
    if let Err(e) = _start_cron_jobs(ctx).await {
        eprintln!("Error starting cron jobs: {:?}", e);
    }
}

async fn _start_cron_jobs(ctx: Context) -> Result<()> {
    let mut jobs: Vec<(Schedule, _)> = vec![(
        Schedule::from_str("0 0 14 * * Mon,Thu")?,
        send_availability_check,
    )];

    jobs.sort_by(|(a, _), (b, _)| {
        let a = a.upcoming(chrono::Utc).next().unwrap_or_default();
        let b = b.upcoming(chrono::Utc).next().unwrap_or_default();
        a.cmp(&b)
    });

    loop {
        let (schedule, action) = jobs.remove(0);

        if let Some(when) = schedule.upcoming(chrono::Utc).next() {
            let now = chrono::Utc::now();
            let delta = when - now;
            let duration = Duration::from_secs(delta.num_seconds() as u64);
            println!("Next job: {:?}", when);
            sleep(duration).await;

            action(&ctx).await?;
            sleep(Duration::from_secs(60)).await;
        }

        jobs.push((schedule, action));
    }
}

async fn send_availability_check(ctx: &Context) -> Result<()> {
    println!("Sending availability check");

    TEAM_LEADS_CHANNEL_ID
        .send_message(
            ctx,
            availability_check_message("Are you available for tomorrow's meeting?"),
        )
        .await?;

    Ok(())
}
