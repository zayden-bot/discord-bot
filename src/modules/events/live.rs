use time::Duration;

use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateScheduledEvent, EditInteractionResponse,
    Permissions, ResolvedOption, ScheduledEventType, Timestamp,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct Live;

#[async_trait]
impl SlashCommand<Error, Postgres> for Live {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        interaction
            .guild_id
            .unwrap()
            .create_scheduled_event(
                ctx,
                CreateScheduledEvent::new(
                    ScheduledEventType::External,
                    "Brad is LIVE",
                    Timestamp::now(),
                )
                .location("https://www.twitch.tv/bradleythebradster")
                .end_time(Timestamp::now().checked_add(Duration::hours(7)).unwrap()),
            )
            .await
            .unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Event successfully created."),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("live")
            .description("Notify the server that Brad is live on Twitch")
            .default_member_permissions(Permissions::CREATE_EVENTS);

        Ok(cmd)
    }
}
