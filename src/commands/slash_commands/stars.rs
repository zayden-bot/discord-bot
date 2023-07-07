use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use crate::models::GoldStar;
use crate::sqlx_lib::get_gold_stars;

pub async fn run<'a>(interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let member = match interaction.data.options.get(0) {
        Some(option) => {
            if let Some(CommandDataOptionValue::User(user, _member)) = option.resolved.as_ref() {
                user
            } else {
                response.interaction_response_data(|message| message.content("Please provide a valid user"));
                return response;
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

    response.interaction_response_data(|message| message.embed(|e| {
        e.title(format!("{}'s Stars", member))
            .field("Number of Stars", stars.number_of_stars, true)
            .field("Given Stars", stars.given_stars, true)
            .field("Received Stars", stars.received_stars, true)
    }));
    response
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