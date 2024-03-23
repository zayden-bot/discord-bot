use crate::models::GoldStar;
use crate::sqlx_lib::{add_star_to_user, get_gold_stars, remove_star_from_author};
use crate::utils::{embed_response, message_response, parse_options};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, ResolvedValue,
};

use crate::Result;

const STARS_TO_GIVE: i32 = 1;

async fn get_user_stars(user_id: u64) -> Result<GoldStar> {
    let user_id = user_id as i64;

    let starts = get_gold_stars(user_id).await?;
    Ok(starts)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    let user = match options.get("member") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => unreachable!("User option is required"),
    };

    if interaction.user.id == user.id {
        message_response(ctx, interaction, "You can't give yourself a star").await?;
        return Ok(());
    }

    let author_stars = get_user_stars(interaction.user.id.get()).await?;
    let member_stars = get_user_stars(user.id.get()).await?;

    let has_free_star = author_stars
        .last_free_star
        .map(|star| star.and_utc().timestamp() >= 86400)
        .unwrap_or(true);

    if author_stars.number_of_stars < STARS_TO_GIVE && !has_free_star {
        message_response(ctx, interaction, "You don't have enough stars to give").await?;
        return Ok(());
    }

    remove_star_from_author(
        interaction.user.id.get() as i64,
        STARS_TO_GIVE,
        has_free_star,
    )
    .await?;

    add_star_to_user(user.id.get() as i64, STARS_TO_GIVE).await?;

    let mut description = format!(
        "{} received a golden star from {} for a total of **{}** stars.",
        user,
        interaction.user,
        member_stars.number_of_stars + STARS_TO_GIVE
    );

    if let Some(ResolvedValue::String(reason)) = options.get("reason") {
        description += &format!("\nReason: {}", reason);
    };

    embed_response(
        ctx,
        interaction,
        CreateEmbed::new()
            .title("⭐ NEW GOLDEN STAR ⭐")
            .description(description),
    )
    .await?;

    Ok(())
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
