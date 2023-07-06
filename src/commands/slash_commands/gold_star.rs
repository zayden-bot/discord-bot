use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;

pub async fn run(interaction: &ApplicationCommandInteraction) -> String {
    return "Work in progress".to_string();

    let author = &interaction.user;

    let member = match interaction.data.options[0].resolved.as_ref().unwrap() {
        CommandDataOptionValue::User(user, _member) => user,
        _ => return "Please provide a valid user".to_string(),
    };
    let reason = interaction.data.options.get(1);

    if author.id == member.id {
        return "You can't give yourself a star!".to_string();
    }

    // TODO: Check if author has an available star
    // TODO: Add star to member
    // TODO: Remove star from author

    let mut response = "⭐ NEW GOLDEN STAR ⭐\n".to_string();
    response += format!("{} recieved a golden star from {} for a total of {} stars.", member, author, 3).as_str();
    if let Some(reason) = reason {
        if let Some(CommandDataOptionValue::String(reason)) = reason.resolved.as_ref() {
            response += format!("\nReason: {}", reason).as_str()
        }
    }

    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("gold_star")
        .description("Give a user a star")
        .create_option(|option| {
            option
                .name("member")
                .description("The member to give a star to")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("reason")
                .description("The reason for giving a star")
                .kind(CommandOptionType::String)
        })
}