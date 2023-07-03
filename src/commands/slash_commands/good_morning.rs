use rand::seq::SliceRandom;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use crate::sqlx_lib::get_good_morning_images;

pub async fn run(_options: &[CommandDataOption]) -> String {
    let good_morning_options = get_good_morning_images().await;

    match good_morning_options.choose(&mut rand::thread_rng()) {
        Some(message) => message.image_url.clone(),
        None => String::from("Error getting good morning image"),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("good_morning").description("Have a CK girl bless your morning")
}