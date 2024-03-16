use crate::models::GoldStar;
use crate::sqlx_lib::{add_star_to_user, create_user, get_gold_stars, remove_star_from_author};
use crate::utils::{embed_response, message_response};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, Message,
};

const STARS_TO_GIVE: i32 = 1;

async fn get_user_stars(user_id: u64) -> Result<GoldStar, String> {
    let user_id = user_id as i64;

    match get_gold_stars(user_id).await {
        Ok(stars) => Ok(stars),
        Err(_) => create_user(user_id, 0, 0)
            .await
            .map_err(|_| "Error creating user".to_string()),
    }
}

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let author = &interaction.user;

    let user_id = match interaction.data.options[0].value.as_user_id() {
        Some(user) => user,
        _ => return message_response(&ctx, interaction, "Please provide a valid user").await,
    };

    if author.id == user_id {
        return message_response(&ctx, interaction, "You can't give yourself a star").await;
    }

    let author_stars = match get_user_stars(author.id.get()).await {
        Ok(stars) => stars,
        Err(_) => {
            return message_response(&ctx, interaction, "Error retrieving author stars").await
        }
    };
    let member_stars = match get_user_stars(user_id.get()).await {
        Ok(stars) => stars,
        Err(_) => {
            return message_response(&ctx, interaction, "Error retrieving member stars").await
        }
    };

    let has_free_star = author_stars
        .last_free_star
        .map(|star| star.and_utc().timestamp() >= 86400)
        .unwrap_or(true);

    if author_stars.number_of_stars < STARS_TO_GIVE && !has_free_star {
        return message_response(&ctx, interaction, "You don't have enough stars to give").await;
    }

    if (remove_star_from_author(author.id.get() as i64, STARS_TO_GIVE, has_free_star).await)
        .is_err()
    {
        return message_response(&ctx, interaction, "Error removing star from author").await;
    }
    if (add_star_to_user(user_id.get() as i64, STARS_TO_GIVE).await).is_err() {
        return message_response(&ctx, interaction, "Error adding star to member").await;
    }

    let mut description = format!(
        "{} received a golden star from {} for a total of **{}** stars.",
        user_id,
        author,
        member_stars.number_of_stars + STARS_TO_GIVE
    );

    if let Some(reason) = interaction
        .data
        .options
        .get(1)
        .and_then(|option| option.value.as_str())
    {
        description += &format!("\nReason: {}", reason)
    }

    embed_response(
        &ctx,
        interaction,
        CreateEmbed::new()
            .title("⭐ NEW GOLDEN STAR ⭐")
            .description(description),
    )
    .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gold_star")
        .description("Give a user a star")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "member",
                "The member to give a star to",
            )
            .required(true),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "reason",
            "The reason for giving a star",
        ))
}
