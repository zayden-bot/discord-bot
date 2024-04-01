use crate::sqlx_lib::get_user_infractions;
use crate::utils::{embed_response, parse_options};
use crate::Result;
use serenity::all::{
    Command, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, Permissions, ResolvedValue,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => user,
        _ => unreachable!("User option is required"),
    };

    let filter = match options.get("filter") {
        Some(ResolvedValue::String(filter)) => filter,
        _ => "recent",
    };

    let infractions = get_user_infractions(user.id.get() as i64, filter == "recent").await?;

    let fields = infractions.into_iter().map(|infraction| {
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

    embed_response(
        ctx,
        interaction,
        CreateEmbed::new()
            .title(format!("Logs for {}", user.name))
            .fields(fields),
    )
    .await?;

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    Command::create_global_command(
        ctx,
        CreateCommand::new("logs")
            .description("Get logs for a user")
            .default_member_permissions(Permissions::MODERATE_MEMBERS)
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "user",
                    "The user to get logs for",
                )
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
            ),
    )
    .await?;

    Ok(())
}
