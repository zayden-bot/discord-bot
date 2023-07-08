use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use crate::sqlx_lib::get_rule;

const RULE_CHANNEL: u64 = 747430712617074718;

pub async fn run<'a>(_ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            response.interaction_response_data(|message| message.content("This command can only be used in a server"));
            return response;
        }
    };

    let rule_id = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::String(id)) => id,
        _ => {
            response.interaction_response_data(|message| message.content("Invalid rule ID"));
            return response;
        }
    };

    let rule = match get_rule(rule_id, guild_id.0 as i64).await {
        Ok(rule) => rule,
        Err(_) => {
            response.interaction_response_data(|message| message.content(format!("Error getting rule: **{}**", rule_id)));
            return response;
        }
    };

    response.interaction_response_data(|message| message.embed(|e| {
        e.title(format!("Rule: {}", rule_id))
            .description(format!("**{}.** {}\n\n**Please read the rest of the rules in <#{}>!**", rule_id, rule, RULE_CHANNEL))
    }));
    response
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