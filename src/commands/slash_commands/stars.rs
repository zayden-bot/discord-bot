use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use crate::models::GoldStar;
use crate::sqlx_lib::get_gold_stars;

pub async fn run(interaction: &ApplicationCommandInteraction) -> String {
    let member = match interaction.data.options.get(0) {
        Some(option) => {
            if let Some(CommandDataOptionValue::User(user, _member)) = option.resolved.as_ref() {
                user
            } else {
                return "Please provide a valid user".to_string();
            }
        },
        None => &interaction.user,
    };

    let stars = match get_gold_stars(member.id.0 as i64).await {
        Ok(stars) => stars,
        Err(_) => GoldStar {
            id: 0,
            number_of_stars: 0,
            given_stars: 0,
            received_stars: 0,
            last_free_star: None,
        }
    };

    format!("{} has **{}** stars", member, stars.number_of_stars)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("stars")
        .description("Get the number of stars a user has.")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to get the stars for.")
                .kind(CommandOptionType::User)
        })
}