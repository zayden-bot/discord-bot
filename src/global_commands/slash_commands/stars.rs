use crate::sqlx_lib::get_gold_stars;
use crate::utils::send_embed;
use crate::Result;
use crate::{models::GoldStar, utils::parse_options};
use serenity::all::{
    Command, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, ResolvedValue,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => &interaction.user,
    };

    let stars = get_gold_stars(user.id.get() as i64)
        .await
        .unwrap_or(GoldStar {
            id: 0,
            number_of_stars: 0,
            given_stars: 0,
            received_stars: 0,
            last_free_star: None,
        });

    send_embed(
        ctx,
        interaction,
        CreateMessage::new().embed(
            CreateEmbed::new()
                .title(format!("{}'s Stars", user.name))
                .field("Number of Stars", stars.number_of_stars.to_string(), true)
                .field("Given Stars", stars.given_stars.to_string(), true)
                .field("Received Stars", stars.received_stars.to_string(), true),
        ),
    )
    .await?;

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    Command::create_global_command(
        ctx,
        CreateCommand::new("stars")
            .description("Get the number of stars a user has.")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "user",
                    "The user to get the stars for.",
                )
                .required(false),
            ),
    )
    .await?;

    Ok(())
}
