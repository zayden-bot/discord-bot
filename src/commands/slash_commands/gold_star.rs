use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use crate::models::GoldStar;
use crate::sqlx_lib::{create_user, get_gold_stars, remove_star_from_author, add_star_to_user};

const STARS_TO_GIVE: i32 = 1;

async fn get_user_stars(user_id: u64) -> Result<GoldStar, String> {
    let user_id = user_id as i64;

    match get_gold_stars(user_id).await {
        Ok(stars) => Ok(stars),
        Err(_) => create_user(user_id, 0, 0).await.map_err(|_| "Error creating user".to_string()),
    }
}

pub async fn run<'a>(_ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let author = &interaction.user;

    let member = if let Some(CommandDataOptionValue::User(user, _member)) = interaction.data.options[0].resolved.as_ref() {
        user
    } else {
        response.interaction_response_data(|message| message.content("Please provide a valid user"));
        return response;
    };
    let reason = interaction.data.options.get(1);

    if author.id == member.id {
        response.interaction_response_data(|message| message.content("You can't give yourself a star!"));
        return response;
    }

    let author_stars = match get_user_stars(author.id.0).await {
        Ok(stars) => stars,
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error retrieving author stars"));
            return response;
        },
    };
    let member_stars = match get_user_stars(member.id.0).await {
        Ok(stars) => stars,
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error retrieving author stars"));
            return response;
        },
    };

    let has_free_star = author_stars.last_free_star.map(|star| star.timestamp() >= 86400).unwrap_or(true);

    if author_stars.number_of_stars < STARS_TO_GIVE && !has_free_star {
        response.interaction_response_data(|message| message.content("You don't have any stars to give!"));
        return response;
    }

    if let Err(_) = remove_star_from_author(author.id.0 as i64, STARS_TO_GIVE, has_free_star).await {
        response.interaction_response_data(|message| message.content("Error removing star from author"));
        return response;
    }
    if let Err(_) = add_star_to_user(member.id.0 as i64, STARS_TO_GIVE).await {
        response.interaction_response_data(|message| message.content("Error adding star to user"));
        return response;
    }

    response.interaction_response_data(|message| {
        message.embed(|e| {
            let mut description = format!("{} received a golden star from {} for a total of **{}** stars.", member, author, member_stars.number_of_stars + STARS_TO_GIVE);

            if let Some(reason) = reason {
                if let Some(CommandDataOptionValue::String(reason)) = reason.resolved.as_ref() {
                    description += &format!("\nReason: {}", reason)
                }
            }

            e.title("⭐ NEW GOLDEN STAR ⭐")
                .description(description)
        })
    });
    return response;
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