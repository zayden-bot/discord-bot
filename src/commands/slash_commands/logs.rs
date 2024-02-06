use crate::models::Infraction;
use crate::sqlx_lib::get_user_infractions;
use crate::utils::{message_response, send_embed};
use chrono::{Months, Utc};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, Message, Permissions,
};

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let user_id = match interaction.data.options[0].value.as_user_id() {
        Some(user_id) => user_id,
        None => return message_response(&ctx, interaction, "Please provide a valid user").await,
    };

    let filter = interaction
        .data
        .options
        .get(1)
        .and_then(|option| option.value.as_str())
        .unwrap_or("recent");

    let mut infractions = match get_user_infractions(user_id.get() as i64).await {
        Ok(user_infractions) => user_infractions,
        Err(_) => return message_response(&ctx, interaction, "Error getting user config").await,
    };

    if filter == "recent" {
        let six_months_age = Utc::now()
            .checked_sub_months(Months::new(6))
            .unwrap()
            .naive_utc();

        infractions = infractions
            .into_iter()
            .filter(|infraction| infraction.created_at >= six_months_age)
            .collect::<Vec<Infraction>>();
    }

    let fields = infractions.iter().map(|infraction| {
        (
            format!("Case #{}", infraction.id),
            format!("**Type:** {}\n", infraction.infraction_type)
                + &format!(
                    "**User:** ({}) {}\n",
                    infraction.user_id, infraction.username
                )
                + &format!(
                    "**Moderator:** ({}) {}\n",
                    infraction.moderator_id, infraction.moderator_username
                )
                + &format!("**Reason:** {}", infraction.reason),
            false,
        )
    });

    send_embed(
        &ctx,
        interaction,
        CreateMessage::new().embed(CreateEmbed::new().fields(fields)),
    )
    .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("logs")
        .description("Get logs for a user")
        .default_member_permissions(Permissions::MODERATE_MEMBERS)
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "The user to get logs for")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "filter",
                "The number of logs to get",
            )
            .add_string_choice("Recent (default)", "recent")
            .add_string_choice("All", "all"),
        )
}
