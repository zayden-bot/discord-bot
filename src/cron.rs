use cron::Schedule;
use serenity::all::{
    ButtonStyle, ChannelId, Context, CreateButton, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, Mentionable, UserId,
};
use serenity::futures::StreamExt;
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::time::sleep_until;

use crate::Result;

const CHANNEL_ID: u64 = 846021706203136030;

pub async fn start_cron_jobs(ctx: Context) -> Result<()> {
    let _result = tokio::spawn(async move { run_at_2pm_mon_thurs(ctx).await });
    Ok(())
}

async fn run_at_2pm_mon_thurs(ctx: Context) -> Result<()> {
    let schedule = Schedule::from_str("0 0 14 * * Mon,Thu")?;
    let mut next = Instant::now();
    let tolerance = Duration::from_secs(1);

    loop {
        if let Some(when) = schedule.upcoming(chrono::Utc).next() {
            let now = chrono::Utc::now();
            if now + tolerance >= when {
                let ctx = ctx.clone();
                tokio::spawn(async move { send_availability_check(ctx).await });

                next += Duration::from_secs(60);
                sleep_until(next.into()).await;
            }
        }
    }
}

pub async fn send_availability_check(ctx: Context) -> Result<()> {
    println!("Sending availability check");

    let channel_id = ChannelId::new(CHANNEL_ID);

    let message = channel_id
        .send_message(
            &ctx,
            CreateMessage::default()
                .embed(
                    CreateEmbed::default()
                        .title("Are you available for tomorrow's meeting?")
                        .field("Attending", "", true)
                        .field("Unavailable", "", true),
                )
                .button(
                    CreateButton::new("available")
                        .label("Attending")
                        .style(ButtonStyle::Success),
                )
                .button(
                    CreateButton::new("unavailable")
                        .label("Unavailable")
                        .style(ButtonStyle::Danger),
                ),
        )
        .await?;

    let mut collector = message
        .await_component_interactions(&ctx)
        .timeout(Duration::from_secs(86400))
        .stream();

    let mut available: Vec<UserId> = Vec::new();
    let mut unavailable: Vec<UserId> = Vec::new();

    while let Some(interaction) = collector.next().await {
        let user_id = interaction.user.id;

        match interaction.data.custom_id.as_str() {
            "available" => {
                if !available.contains(&user_id) {
                    available.push(user_id);
                }
                unavailable.retain(|&x| x != user_id);
            }
            "unavailable" => {
                if !unavailable.contains(&user_id) {
                    unavailable.push(user_id);
                }
                available.retain(|&x| x != user_id);
            }
            _ => unreachable!("Invalid custom_id"),
        }

        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::default().embed(
                        CreateEmbed::default()
                            .title("Are you available for tomorrow's meeting?")
                            .field(
                                "Attending",
                                available.iter().fold(String::new(), |mut output, user_id| {
                                    output.push_str(&format!("\n{}", user_id.mention()));
                                    output
                                }),
                                true,
                            )
                            .field(
                                "Unavailable",
                                unavailable
                                    .iter()
                                    .fold(String::new(), |mut output, user_id| {
                                        output.push_str(&format!("\n{}", user_id.mention()));
                                        output
                                    }),
                                true,
                            ),
                    ),
                ),
            )
            .await?;
    }

    todo!()
}
