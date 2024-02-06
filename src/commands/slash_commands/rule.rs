use crate::sqlx_lib::get_rule;
use crate::utils::{message_response, send_embed};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, Message,
};

const RULE_CHANNEL: u64 = 747430712617074718;

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return message_response(
                &ctx,
                interaction,
                "This command can only be used in a server",
            )
            .await
        }
    };

    let rule_id = match interaction.data.options[0].value.as_str() {
        Some(id) => id,
        _ => return message_response(&ctx, interaction, "Invalid rule ID").await,
    };

    let rule = match get_rule(rule_id, guild_id.get() as i64).await {
        Ok(rule) => rule,
        Err(_) => return message_response(&ctx, interaction, "Error getting rule").await,
    };

    send_embed(
        &ctx,
        interaction,
        CreateMessage::new().embed(
            CreateEmbed::new()
                .title(format!("Rule: {}", rule_id))
                .description(format!(
                    "**{}.** {}\n\n**Please read the rest of the rules in <#{}>!**",
                    rule_id, rule, RULE_CHANNEL
                )),
        ),
    )
    .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("rule")
        .description("Get a rule")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "rule_id", "The ID of the rule")
                .required(true),
        )
}
