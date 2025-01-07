use std::str::FromStr;

use async_trait::async_trait;
use cron::Schedule;
use serenity::all::Context;

use crate::guild_commands::college_kings::availability_check::availability_check_message;
use crate::guilds::college_kings_team::TEAM_LEADS_CHANNEL_ID;
use crate::Result;

use super::CronJob;

pub struct AvailabilityCheck;

#[async_trait]
impl CronJob for AvailabilityCheck {
    fn schedule(&self) -> Schedule {
        Schedule::from_str("0 0 14 * * Mon,Thu").unwrap()
    }

    async fn action(&self, ctx: &Context) -> Result<()> {
        println!("Sending availability check");

        TEAM_LEADS_CHANNEL_ID
            .send_message(
                ctx,
                availability_check_message("Are you available for tomorrow's meeting?"),
            )
            .await?;

        Ok(())
    }
}
