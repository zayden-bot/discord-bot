use cron::Schedule;
use serenity::all::{
    ButtonStyle, ChannelId, Context, CreateButton, CreateEmbed, CreateMessage, Mentionable, RoleId,
};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

use crate::Result;

const CHANNEL_ID: ChannelId = ChannelId::new(846021706203136030);
const ROLE_ID: RoleId = RoleId::new(836275726352646176);

pub async fn start_cron_jobs(ctx: Context) -> Result<()> {
    tokio::spawn(async move { run_at_2pm_mon_thurs(ctx).await }).await??;

    Ok(())
}

async fn run_at_2pm_mon_thurs(ctx: Context) -> Result<()> {
    let schedule = Schedule::from_str("0 0 14 * * Mon,Thu")?;

    loop {
        if let Some(when) = schedule.upcoming(chrono::Utc).next() {
            let now = chrono::Utc::now();
            let delta = when - now;
            let duration = Duration::from_secs(delta.num_seconds() as u64);
            println!("run_at_2pm_mon_thurs: {:?}", when);
            sleep(duration).await;

            let ctx_clone = ctx.clone();
            tokio::spawn(async move { send_availability_check(ctx_clone).await }).await??;
        }
    }
}

async fn send_availability_check(ctx: Context) -> Result<()> {
    println!("Sending availability check");

    CHANNEL_ID
        .send_message(
            &ctx,
            CreateMessage::default()
                .content(ROLE_ID.mention().to_string())
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
