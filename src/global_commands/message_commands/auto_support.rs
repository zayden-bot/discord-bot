use serenity::all::{
    AutoArchiveDuration, ChannelType, Context, CreateAttachment, CreateMessage, CreateThread,
    DiscordJsonError, ErrorResponse, Message,
};
use serenity::http::HttpError::UnsuccessfulRequest;

use crate::sqlx_lib::{
    get_support_channel_ids, get_support_role_ids, get_support_thead_id, update_support_thread_id,
    PostgresPool,
};
use crate::utils::support::get_thread_name;
use crate::{Error, Result};

async fn get_attachments(msg: &Message) -> serenity::Result<Vec<CreateAttachment>> {
    let mut attachments: Vec<CreateAttachment> = Vec::new();
    for attachment in &msg.attachments {
        attachments.push(CreateAttachment::bytes(
            attachment.download().await?,
            attachment.filename.clone(),
        ));
    }
    Ok(attachments)
}

pub async fn run(ctx: &Context, msg: &Message) -> Result<()> {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let pool = {
        let data = ctx.data.read().await;
        data.get::<PostgresPool>()
            .expect("PostgresPool should exist in data.")
            .clone()
    };

    let support_channel_ids = get_support_channel_ids(&pool, guild_id.get()).await?;
    if !support_channel_ids.contains(&(msg.channel_id.get() as i64)) {
        return Ok(());
    }

    let guild_roles = guild_id.roles(&ctx).await?;

    let support_role_ids = get_support_role_ids(&pool, guild_id.get()).await?;
    let support_role = guild_roles
        .into_iter()
        .find(|(role_id, _)| role_id.get() == (support_role_ids[0] as u64))
        .ok_or_else(|| Error::NoRole)?
        .1;

    if msg.member(&ctx).await?.roles.contains(&support_role.id) {
        return Ok(());
    }

    let attachments = get_attachments(msg).await?;

    let thread_id = get_support_thead_id(&pool, guild_id.get())
        .await
        .unwrap_or(0)
        + 1;
    update_support_thread_id(&pool, guild_id.get(), thread_id).await?;

    let thread_name = get_thread_name(thread_id, &msg.author.name, &msg.content);
    // let files = self.0.attachments.as_mut().map_or(Vec::new(), |a| a.take_files());
    let thread = msg
        .channel_id
        .create_thread(
            &ctx,
            CreateThread::new(thread_name)
                .kind(ChannelType::PrivateThread)
                .auto_archive_duration(AutoArchiveDuration::OneWeek),
        )
        .await?;

    thread
        .say(&ctx, format!("{} {} wrote:", support_role, msg.author))
        .await?;

    let chunks: Vec<String> = msg
        .content
        .chars()
        .collect::<Vec<char>>()
        .chunks(2000)
        .map(|c| c.iter().collect())
        .collect();

    if chunks.is_empty() {
        thread
            .send_files(&ctx, attachments, CreateMessage::default())
            .await?;
    } else {
        for chunk in &chunks[..chunks.len() - 1] {
            thread.say(&ctx, chunk).await?;
        }

        let last_chunk = chunks.last().expect("Chunks is not empty");
        thread
            .send_files(&ctx, attachments, CreateMessage::new().content(last_chunk))
            .await?;
    }

    match msg.delete(&ctx).await {
        // 10008: Unknown Message
        Err(serenity::Error::Http(UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 10008, .. },
            ..
        }))) => {}
        result => {
            result?;
        }
    }

    Ok(())
}
