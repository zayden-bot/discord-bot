use crate::sqlx_lib::{get_gold_stars, get_pool};
use crate::utils::embed_response;
use crate::utils::parse_options;
use crate::Result;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, ResolvedValue,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => &interaction.user,
    };

    let pool = get_pool(ctx).await?;

    let stars = get_gold_stars(&pool, user.id.get())
        .await
        .unwrap_or_default();

    embed_response(
        ctx,
        interaction,
        CreateEmbed::new()
            .title(format!("{}'s Stars", user.name))
            .field("Number of Stars", stars.number_of_stars.to_string(), true)
            .field("Given Stars", stars.given_stars.to_string(), true)
            .field("Received Stars", stars.received_stars.to_string(), true),
    )
    .await?;

    Ok(())
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
