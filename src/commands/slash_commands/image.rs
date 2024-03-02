use crate::image_cache::ImageCache;
use chrono::{Duration, Local};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, EditAttachments,
    EditInteractionResponse, Message,
};

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    interaction.defer(&ctx).await.expect("Failed to defer");

    let mut data = ctx.data.write().await;
    let image_cache = data.get_mut::<ImageCache>().unwrap();

    if Local::now().naive_utc() > image_cache.last_update + Duration::hours(1) {
        image_cache.update().await;
    };

    let entries = [
        image_cache.good_night_images.as_slice(),
        image_cache.good_morning_images.as_slice(),
    ]
    .concat();

    let image_path = entries.choose(&mut thread_rng()).unwrap();
    let file_name = image_path.file_name().unwrap().to_str().unwrap();

    interaction
        .edit_response(
            &ctx,
            EditInteractionResponse::new()
                .embed(
                    CreateEmbed::new()
                        .title(format!("Good Night, {}!", interaction.user.name))
                        .attachment(file_name),
                )
                .attachments(
                    EditAttachments::new().add(CreateAttachment::path(image_path).await.unwrap()),
                ),
        )
        .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("image").description("Get a random image from the image cache")
}
