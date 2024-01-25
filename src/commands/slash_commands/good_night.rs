use crate::image_cache::ImageCache;
use chrono::{Duration, Local};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub async fn run(ctx: Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let mut data = ctx.data.write().await;
    let image_cache = data.get_mut::<ImageCache>().unwrap();

    let entries = if Local::now().naive_utc() < image_cache.last_update + Duration::hours(1) {
        image_cache.good_night_images.clone()
    } else {
        image_cache.update().await;
        image_cache.good_night_images.clone()
    };

    let image_path = entries.choose(&mut thread_rng()).unwrap();
    let file_name = image_path.file_name().unwrap().to_str().unwrap();

    interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .add_embed(
                        CreateEmbed::new()
                            .title(format!("Good Night, {}!", interaction.user.name))
                            .attachment(file_name),
                    )
                    .add_file(CreateAttachment::path(image_path).await.unwrap()),
            ),
        )
        .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("good_night").description("Have a CK girl wish you good night")
}
