use std::time::Duration;

use crate::guilds::ServersTable;
use crate::image_cache::ImageCache;
use crate::sqlx_lib::PostgresPool;
use crate::utils::message_response;
use crate::{Error, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, EditAttachments,
    EditInteractionResponse, Ready, UserId,
};
use serenity::prelude::TypeMapKey;

pub struct GoodNightLockedUsers;

impl TypeMapKey for GoodNightLockedUsers {
    type Value = Vec<UserId>;
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(&ctx).await.unwrap();

    let pool = PostgresPool::get(ctx).await;

    let mut data = ctx.data.write().await;
    let locked_users = data
        .get_mut::<GoodNightLockedUsers>()
        .ok_or_else(|| Error::DataNotFound)?;

    let user_id = interaction.user.id;

    let general_channel_id = ServersTable::get_row(&pool, interaction.guild_id.unwrap())
        .await
        .unwrap()
        .unwrap()
        .get_general_channel_id()
        .unwrap();

    if interaction.channel_id == general_channel_id {
        if locked_users.contains(&user_id) {
            message_response(
                ctx,
                interaction,
                "You have already used this command today.",
            )
            .await
            .unwrap();
            return Ok(());
        }

        locked_users.push(user_id);
    }

    let image_cache = data
        .get::<ImageCache>()
        .ok_or_else(|| Error::DataNotFound)?;

    let entries = &image_cache.good_night_images;

    let image_path = entries
        .choose(&mut thread_rng())
        .ok_or_else(|| Error::NoImage)?;
    let file_name = image_path
        .file_name()
        .ok_or_else(|| Error::NoFileName)?
        .to_str()
        .ok_or_else(|| Error::NoFileName)?;

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
        .unwrap();

    if interaction.channel_id == general_channel_id {
        tokio::spawn({
            let ctx = ctx.clone();
            async move {
                tokio::time::sleep(Duration::from_secs(60 * 60 * 8)).await;
                let mut data = ctx.data.write().await;
                if let Some(locked_users) = data.get_mut::<GoodNightLockedUsers>() {
                    locked_users.retain(|x| *x != user_id);
                }
            }
        });
    }

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("goodnight").description("Have a CK girl wish you good night");

    Ok(command)
}
