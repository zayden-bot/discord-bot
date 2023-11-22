use crate::models::GoldStar;
use crate::sqlx_lib::get_gold_stars;
use crate::utils::respond_with_embed;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let user_id = match interaction
        .data
        .options
        .get(0)
        .and_then(|option| option.value.as_user_id())
    {
        Some(user_id) => user_id,
        None => interaction.user.id,
    };

    let stars = match get_gold_stars(user_id.get() as i64).await {
        Ok(stars) => stars,
        Err(_) => GoldStar {
            id: 0,
            number_of_stars: 0,
            given_stars: 0,
            received_stars: 0,
            last_free_star: None,
        },
    };

    respond_with_embed(
        ctx,
        interaction,
        CreateEmbed::new()
            .title(format!(
                "{}'s Stars",
                user_id.to_user(&ctx).await.unwrap().name
            ))
            .field("Number of Stars", stars.number_of_stars.to_string(), true)
            .field("Given Stars", stars.given_stars.to_string(), true)
            .field("Received Stars", stars.received_stars.to_string(), true),
    )
    .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stars")
        .description("Get the number of stars a user has.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to get the stars for.",
            )
            .required(false),
        )
}
