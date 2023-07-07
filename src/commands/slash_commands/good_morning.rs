use rand::seq::SliceRandom;
use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::{ApplicationCommandInteraction};
use serenity::prelude::Context;
use crate::sqlx_lib::get_good_morning_images;

pub async fn run<'a>(_ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let good_morning_options = get_good_morning_images().await;

    let good_morning_image = match good_morning_options.choose(&mut rand::thread_rng()) {
        Some(message) => &message.image_url,
        None => {
            response.interaction_response_data(|message| message.content("Error getting good morning image"));
            return response;
        },
    };

    response.interaction_response_data(|message| message.embed(|e| {
        e.title(format!("Good Morning, {}!", interaction.user.name))
            .image(good_morning_image)
    }));
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("good_morning").description("Have a CK girl bless your morning")
}