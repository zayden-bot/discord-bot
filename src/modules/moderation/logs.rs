use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, Permissions, Ready, ResolvedOption, ResolvedValue,
};
use zayden_core::{parse_options, SlashCommand};

use crate::sqlx_lib::PostgresPool;
use crate::utils::embed_response;
use crate::{Error, Result};

use super::InfractionRow;

pub struct Logs;

#[async_trait]
impl SlashCommand<Error> for Logs {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        let options = interaction.data.options();
        let options = parse_options(&options);

        let user = match options.get("user") {
            Some(ResolvedValue::User(user, _)) => *user,
            _ => unreachable!("User option is required"),
        };

        let filter = match options.get("filter") {
            Some(ResolvedValue::String(filter)) => filter,
            _ => "recent",
        };

        let pool = PostgresPool::get(ctx).await;

        let infractions =
            InfractionRow::user_infractions(&pool, user.id.get(), filter == "recent").await?;

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

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("logs")
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
            );

        Ok(command)
    }
}
