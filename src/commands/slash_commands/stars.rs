use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use crate::models::GoldStar;
use crate::sqlx_lib::get_gold_stars;
use crate::utils::{respond_with_embed, respond_with_message};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let member = match interaction.data.options.get(0) {
        Some(option) => match option.resolved.as_ref() {
            Some(CommandDataOptionValue::User(user, _member)) => user,
            _ => return respond_with_message(ctx, interaction, "Please provide a valid user").await,
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

    respond_with_embed(ctx, interaction, |e| {
        e.title(format!("{}'s Stars", member))
            .field("Number of Stars", stars.number_of_stars, true)
            .field("Given Stars", stars.given_stars, true)
            .field("Received Stars", stars.received_stars, true)
    }).await
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