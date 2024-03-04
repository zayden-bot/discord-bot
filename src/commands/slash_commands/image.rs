use std::path::PathBuf;

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

    let guild_roles = interaction
        .guild_id
        .unwrap()
        .to_partial_guild(&ctx)
        .await?
        .roles;
    let member_roles = &interaction.member.as_ref().unwrap().roles;

    let entries: Vec<&PathBuf> = member_roles
        .iter()
        .map(|role_id| {
            let role = guild_roles.get(role_id).unwrap();
            &role.name
        })
        .filter_map(|name| {
            if let Some(name) = name.split('\'').next() {
                image_cache.character_map.get(&name.to_lowercase())
            } else {
                None
            }
        })
        .flatten()
        .collect();

    let image_path = match entries.choose(&mut thread_rng()) {
        Some(path) => path,
        None => {
            let images = &image_cache.good_morning_images;
            images.choose(&mut thread_rng()).unwrap()
        }
    };
    let file_name = image_path.file_name().unwrap().to_str().unwrap();

    interaction
        .edit_response(
            &ctx,
            EditInteractionResponse::new()
                .embed(CreateEmbed::new().attachment(file_name))
                .attachments(
                    EditAttachments::new().add(CreateAttachment::path(image_path).await.unwrap()),
                ),
        )
        .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("image").description("Get a random image from the image cache")
}
