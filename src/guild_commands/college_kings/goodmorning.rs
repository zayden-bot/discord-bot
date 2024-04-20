use std::time::Duration;

use crate::{
    guilds::college_kings::GENERAL_CHANNEL_ID, utils::message_response, Error, ImageCache, Result,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::{
    all::{
        CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, EditAttachments,
        EditInteractionResponse, UserId,
    },
    prelude::TypeMapKey,
};

pub struct GoodMorningLockedUsers;

impl TypeMapKey for GoodMorningLockedUsers {
    type Value = Vec<UserId>;
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(&ctx).await?;

    let mut data = ctx.data.write().await;
    let locked_users = data
        .get_mut::<GoodMorningLockedUsers>()
        .ok_or_else(|| Error::DataNotFound)?;

    let user_id = interaction.user.id;

    if locked_users.contains(&user_id) {
        message_response(
            ctx,
            interaction,
            "You have already used this command today.",
        )
        .await?;
        return Ok(());
    } else if interaction.channel_id == GENERAL_CHANNEL_ID {
        locked_users.push(user_id);
    }

    let image_cache = data
        .get::<ImageCache>()
        .ok_or_else(|| Error::DataNotFound)?;

    let entries = &image_cache.good_morning_images;

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
                        .title(format!("Good Morning, {}!", interaction.user.name))
                        .attachment(file_name),
                )
                .attachments(EditAttachments::new().add(CreateAttachment::path(image_path).await?)),
        )
        .await?;

    if interaction.channel_id == GENERAL_CHANNEL_ID {
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

pub fn register() -> CreateCommand {
    CreateCommand::new("goodmorning").description("Have a CK girl bless your morning")
}
