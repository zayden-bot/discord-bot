use async_trait::async_trait;
use serenity::all::{
    ButtonStyle, CommandInteraction, CommandOptionType, Context, CreateButton, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateMessage, EditInteractionResponse, Mentionable, Ready,
    ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{parse_options, SlashCommand};

use crate::guilds::college_kings_team::{
    STEVE_USER_ID, TEAM_LEADERS_ROLE_ID, TEAM_LEADS_CHANNEL_ID,
};
use crate::{Error, Result};

pub struct AvailabilityCheck;

#[async_trait]
impl SlashCommand<Error, Postgres> for AvailabilityCheck {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let mut options = parse_options(options);

        let title = match options.remove("title") {
            Some(ResolvedValue::String(title)) => title,
            _ => "Availability Check",
        };

        TEAM_LEADS_CHANNEL_ID
            .send_message(ctx, availability_check_message(title))
            .await
            .unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Availability check sent"),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("availability_check")
            .description("Check availability for team leads")
            .add_option(CreateCommandOption::new(
                CommandOptionType::String,
                "title",
                "Title of the availability check",
            ));

        Ok(command)
    }
}

pub fn availability_check_message(title: impl Into<String>) -> CreateMessage {
    CreateMessage::default()
        .content(TEAM_LEADERS_ROLE_ID.mention().to_string())
        .embed(
            CreateEmbed::default()
                .title(title)
                .field("Attending", "", true)
                .field("Unavailable", STEVE_USER_ID.mention().to_string(), true),
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
        )
}
