use crate::sqlx_lib::get_good_night_images;
use crate::utils::{respond_with_embed, respond_with_message};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let good_night_options = get_good_night_images().await;

    let good_night_option = good_night_options.choose(&mut thread_rng());

    let good_night_image = match good_night_option {
        Some(message) => &message.image_url,
        None => {
            return respond_with_message(ctx, interaction, "Error getting good night image").await
        }
    };

    respond_with_embed(
        ctx,
        interaction,
        CreateEmbed::new()
            .title(format!("Good Night, {}!", interaction.user.name))
            .image(good_night_image),
    )
    .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("good_night").description("Have a CK girl wish you good night")
}
