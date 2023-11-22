use crate::sqlx_lib::get_good_morning_images;
use crate::utils::{respond_with_embed, respond_with_message};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let good_morning_options = get_good_morning_images().await;

    let good_morning_option = good_morning_options.choose(&mut thread_rng());

    let good_morning_image = match good_morning_option {
        Some(message) => &message.image_url,
        None => {
            return respond_with_message(ctx, interaction, "Error getting good morning image").await
        }
    };

    respond_with_embed(
        ctx,
        interaction,
        CreateEmbed::new()
            .title(format!("Good Morning, {}!", interaction.user.name))
            .image(good_morning_image),
    )
    .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("good_morning").description("Have a CK girl bless your morning")
}
