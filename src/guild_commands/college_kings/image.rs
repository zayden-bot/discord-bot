use std::path::PathBuf;

use crate::image_cache::ImageCache;
use crate::{Error, Result};
use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;
use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, EditAttachments,
    EditInteractionResponse, Ready, Role,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(&ctx).await.unwrap();

    let guild_roles = interaction
        .guild_id
        .ok_or_else(|| Error::NotInGuild)?
        .roles(&ctx)
        .await
        .unwrap();

    let member_roles: Vec<&Role> = interaction
        .member
        .as_ref()
        .ok_or_else(|| Error::NoMember)?
        .roles
        .iter()
        .map(|role| -> Result<&Role> {
            guild_roles
                .get(role)
                .ok_or_else(|| Error::RoleNotFound(role.get()))
        })
        .collect::<Result<_>>()?;

    let image_map = {
        let data = ctx.data.read().await;
        let image_cache = data
            .get::<ImageCache>()
            .ok_or_else(|| Error::DataNotFound)?;
        image_cache.character_map.clone()
    };

    let entries: Vec<&PathBuf> = member_roles
        .into_iter()
        .filter_map(|role| {
            let name = role.name.split('\'').next()?.to_lowercase();
            image_map.get(name.as_str())
        })
        .flatten()
        .collect();

    let image_path = match entries.choose(&mut thread_rng()) {
        Some(path) => *path,
        None => image_map
            .values()
            .flat_map(|v| v.iter())
            .choose(&mut thread_rng())
            .ok_or_else(|| Error::NoImage)?,
    };
    let file_name = image_path
        .file_name()
        .ok_or_else(|| Error::NoFileName)?
        .to_str()
        .ok_or_else(|| Error::NoFileName)?;

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
        .unwrap();

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command =
        CreateCommand::new("image").description("Get a random image from the image cache");

    Ok(command)
}
