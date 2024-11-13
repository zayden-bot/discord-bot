use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, Mentionable, Ready, ResolvedValue,
};
use zayden_core::parse_options;

use crate::guilds::ServersTableError;
use crate::sqlx_lib::{get_rule, PostgresPool};
use crate::utils::embed_response;
use crate::{guilds, Error, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NotInGuild)?;

    let options = interaction.data.options();
    let options = parse_options(&options);

    let rule_id = match options.get("id") {
        Some(ResolvedValue::String(id)) => *id,
        _ => unreachable!("Rule ID is required"),
    };

    let pool = PostgresPool::get(ctx).await;

    let rule = get_rule(&pool, rule_id, guild_id.get()).await?;

    let rules_channel_id = guilds::ServersTable::get_row(&pool, guild_id.get())
        .await?
        .ok_or(ServersTableError::ServerNotFound)?
        .get_rules_channel_id()?;

    embed_response(
        ctx,
        interaction,
        CreateEmbed::new()
            .title(format!("Rule: {}", rule_id))
            .description(format!(
                "**{}.** {}\n\n**Please read the rest of the rules in {}!**",
                rule_id,
                rule,
                rules_channel_id.mention()
            )),
    )
    .await?;

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("rule")
        .description("Get a rule")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "id", "The ID of the rule")
                .required(true),
        );

    Ok(command)
}
