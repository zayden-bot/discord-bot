use cron::Schedule;
use serenity::all::{
    ButtonStyle, ChannelId, Context, CreateButton, CreateEmbed, CreateMessage, Mentionable, RoleId,
};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::{sleep_until, Instant};

use crate::Result;

const CHANNEL_ID: u64 = 846021706203136030;
const ROLE_ID: u64 = 836275726352646176;

pub async fn start_cron_jobs(ctx: Context) -> Result<()> {
    let result = tokio::spawn(async move { run_at_2pm_mon_thurs(ctx).await });
    println!("Cron jobs started: {:?}", result.await);

    Ok(())
}

async fn run_at_2pm_mon_thurs(ctx: Context) -> Result<()> {
    let schedule = Schedule::from_str("0 0 14 * * Mon,Thu")?;
    let tolerance = Duration::from_secs(1);

    loop {
        if let Some(when) = schedule.upcoming(chrono::Utc).next() {
            let now = chrono::Utc::now();
            if now + tolerance >= when {
                let ctx = ctx.clone();
                tokio::spawn(async move { send_availability_check(ctx).await });

                let delta = when - now;
                let duration = Duration::from_secs(delta.num_seconds() as u64);
                sleep_until(Instant::now() + duration).await;
            }
        }
    }
}

async fn send_availability_check(ctx: Context) -> Result<()> {
    println!("Sending availability check");

    let channel_id = ChannelId::new(CHANNEL_ID);

    channel_id
        .send_message(
            &ctx,
            CreateMessage::default()
                .content(RoleId::new(ROLE_ID).mention().to_string())
                .embed(
                    CreateEmbed::default()
                        .title("Are you available for tomorrow's meeting?")
                        .field("Attending", "", true)
                        .field("Unavailable", "", true),
                )
                .button(
                    CreateButton::new("cron_available")
                        .label("Attending")
                        .style(ButtonStyle::Success),
                )
                .button(
                    CreateButton::new("cron_unavailable")
                        .label("Unavailable")
                        .style(ButtonStyle::Danger),
                ),
        )
        .await?;

    Ok(())
}
