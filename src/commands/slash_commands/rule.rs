use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use crate::sqlx_lib::get_rule;
use crate::utils::{respond_with_embed, respond_with_message};

const RULE_CHANNEL: u64 = 747430712617074718;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let rule_id = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::String(id)) => id,
        _ => return respond_with_message(ctx, interaction, "Invalid rule ID").await,
    };

    let rule = match get_rule(rule_id, guild_id.0 as i64).await {
        Ok(rule) => rule,
        Err(_) => return respond_with_message(ctx, interaction, "Error getting rule").await,
    };

    respond_with_embed(ctx, interaction, |e| {
        e.title(format!("Rule: {}", rule_id))
            .description(format!("**{}.** {}\n\n**Please read the rest of the rules in <#{}>!**", rule_id, rule, RULE_CHANNEL))
    }).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("rule")
        .description("Get a rule")
        .create_option(|option| {
            option.name("rule_id")
                .description("The ID of the rule")
                .kind(CommandOptionType::String)
                .required(true)
        })
}