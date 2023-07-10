use rand::seq::SliceRandom;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction};
use serenity::prelude::Context;
use crate::sqlx_lib::get_good_morning_images;
use crate::utils::{respond_with_embed, respond_with_message};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let good_morning_options = get_good_morning_images().await;

    let good_morning_option = good_morning_options.choose(&mut rand::thread_rng());

    let good_morning_image = match good_morning_option {
        Some(message) => &message.image_url,
        None => return respond_with_message(ctx, interaction, "Error getting good morning image").await,
    };

    respond_with_embed(ctx, interaction, |e| {
        e.title(format!("Good Morning, {}!", interaction.user.name))
            .image(good_morning_image)
    }).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("good_morning").description("Have a CK girl bless your morning")
}