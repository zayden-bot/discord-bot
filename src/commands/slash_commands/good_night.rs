use rand::seq::SliceRandom;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use crate::sqlx_lib::get_good_night_images;

pub async fn run(_interaction: &ApplicationCommandInteraction) -> String {
    let good_night_options = get_good_night_images().await;

    match good_night_options.choose(&mut rand::thread_rng()) {
        Some(message) => message.image_url.clone(),
        None => String::from("Error getting good morning image"),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("good_night").description("Have a CK girl wish you good night")
}