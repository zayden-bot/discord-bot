use std::time::Duration;

use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, EditAttachments,
    EditInteractionResponse, Ready, UserId,
};
use serenity::prelude::TypeMapKey;

use crate::guilds::ServersTable;
use crate::sqlx_lib::PostgresPool;
use crate::utils::message_response;
use crate::{ImageCache, Result};

pub struct GoodMorningLockedUsers;

impl TypeMapKey for GoodMorningLockedUsers {
    type Value = Vec<UserId>;
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(&ctx).await.unwrap();

    let pool = PostgresPool::get(ctx).await;

    let mut data = ctx.data.write().await;
    let locked_users = data.get_mut::<GoodMorningLockedUsers>().unwrap();

    let user_id = interaction.user.id;

    let row = ServersTable::get_row(&pool, interaction.guild_id.unwrap())
        .await
        .unwrap()
        .unwrap();

    let general_channel_id = row.get_general_channel_id()?;

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

    let image_cache = data.get::<ImageCache>().unwrap();

    let entries = &image_cache.good_morning_images;

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
        .unwrap();

    if interaction.channel_id == general_channel_id {
        tokio::spawn({
            let ctx = ctx.clone();
            async move {
                tokio::time::sleep(Duration::from_secs(60 * 60 * 8)).await;
                let mut data = ctx.data.write().await;
                if let Some(locked_users) = data.get_mut::<GoodMorningLockedUsers>() {
                    locked_users.retain(|x| *x != user_id);
                }
            }
        });
    }

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command =
        CreateCommand::new("goodmorning").description("Have a CK girl bless your morning");

    Ok(command)
}
