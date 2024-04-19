use std::time::Duration;

use crate::{guilds::college_kings::GUILD_ID, Error, ImageCache, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, EditAttachments,
    EditInteractionResponse,
};

pub struct GoodMorningLockedUsers;

impl TypeMapKey for LockedUsers {
    type Value = Vec<UserId>;
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(&ctx).await?;

    let data = ctx.data.read().await;
    let locked_users = data
        .get::<LockedUsers>()
        .ok_or_else(|| Error::DataNotFound)?;

    if locked_users.contains(&interaction.user.id) {
        message_response(
            ctx,
            interaction,
            "You have already used this command today.",
        )
        .await?;
        return Ok(());
    } else if interaction.channel_id == GENERAL_CHANNEL_ID {
        let data = ctx.data.write().await;
        let locked_users = data
            .get_mut::<LockedUsers>()
            .ok_or_else(|| Error::DataNotFound)?;
        locked_users.push(interaction.user.id);
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
        tokio::time::sleep(Duration::from_secs(60 * 60 * 8)).await;
        let data = ctx.data.write().await;
        let locked_users = data
            .get_mut::<LockedUsers>()
            .ok_or_else(|| Error::DataNotFound)?;

        if let Some(pos) = locked_users.iter().position(|x| *x == interaction.user.id) {
            locked_users.remove(pos);
        }
    }

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    GUILD_ID
        .create_command(
            ctx,
            CreateCommand::new("goodmorning").description("Have a CK girl bless your morning"),
        )
        .await?;

    Ok(())
}
