use rand::seq::SliceRandom;
use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::sqlx_lib::get_good_night_images;

pub async fn run<'a>(_ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let good_night_options = get_good_night_images().await;

    let good_night_image = match good_night_options.choose(&mut rand::thread_rng()) {
        Some(message) => &message.image_url,
        None => {
            response.interaction_response_data(|message| message.content("Error getting good night image"));
            return response;
        },
    };

    response.interaction_response_data(|message| message.embed(|e| {
        e.title(format!("Good Morning, {}!", interaction.user.name))
            .image(good_night_image)
    }));
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("good_night").description("Have a CK girl wish you good night")
}