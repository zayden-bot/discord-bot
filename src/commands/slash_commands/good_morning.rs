use crate::ImageCache;
use chrono::{Local, TimeDelta};
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

    let entries =
        if Local::now().naive_utc() < image_cache.last_update + TimeDelta::try_hours(1).unwrap() {
            image_cache.good_morning_images.clone()
        } else {
            image_cache.update().await;
            image_cache.good_morning_images.clone()
        };

    let image_path = entries.choose(&mut thread_rng()).unwrap();
    let file_name = image_path.file_name().unwrap().to_str().unwrap();

    interaction
        .edit_response(
            &ctx,
            EditInteractionResponse::new()
                .embed(
                    CreateEmbed::new()
                        .title(format!("Good Morning, {}!", interaction.user.name))
                        .attachment(file_name),
                )
                .attachments(
                    EditAttachments::new().add(CreateAttachment::path(image_path).await.unwrap()),
                ),
        )
        .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("good_morning").description("Have a CK girl bless your morning")
}
