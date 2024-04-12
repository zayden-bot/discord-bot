use serenity::all::{
    ButtonStyle, CommandInteraction, CommandOptionType, Context, CreateButton, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateMessage, Mentionable, ResolvedValue,
};

use crate::guilds::college_kings_team::{GUILD_ID, TEAM_LEADS_CHANNEL_ID, TEAM_LEADS_ROLE_ID};
use crate::utils::{message_response, parse_options};
use crate::Result;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer_ephemeral(ctx).await?;

    let options = interaction.data.options();
    let options = parse_options(&options);

    let title = match options.get("title") {
        Some(ResolvedValue::String(title)) => title,
        _ => "Availability Check",
    };

    TEAM_LEADS_CHANNEL_ID
        .send_message(ctx, availability_check_message(title))
        .await?;

    message_response(ctx, interaction, "Availability check sent").await?;

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    GUILD_ID
        .create_command(
            ctx,
            CreateCommand::new("availability_check")
                .description("Check availability for team leads")
                .add_option(CreateCommandOption::new(
                    CommandOptionType::String,
                    "title",
                    "Title of the availability check",
                )),
        )
        .await?;

    Ok(())
}

pub fn availability_check_message(title: impl Into<String>) -> CreateMessage {
    CreateMessage::default()
        .content(TEAM_LEADS_ROLE_ID.mention().to_string())
        .embed(
            CreateEmbed::default()
                .title(title)
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
        )
}
