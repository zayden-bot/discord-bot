use rand::seq::SliceRandom;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::sqlx_lib::get_good_night_images;
use crate::utils::{respond_with_embed, respond_with_message};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let good_night_options = get_good_night_images().await;

    let good_night_option = good_night_options.choose(&mut rand::thread_rng());

    let good_night_image = match good_night_option {
        Some(message) => &message.image_url,
        None => return respond_with_message(ctx, interaction, "Error getting good night image").await,
    };

    respond_with_embed(ctx, interaction, |e| {
        e.title(format!("Good Night, {}!", interaction.user.name))
            .image(good_night_image)
    }).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("good_night").description("Have a CK girl wish you good night")
}