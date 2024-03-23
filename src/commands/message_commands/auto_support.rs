use crate::sqlx_lib::{
    get_support_channel_ids, get_support_role_ids, get_support_thead_id, update_support_thread_id,
};
use crate::utils::support::get_thread_name;
use crate::{Error, Result};
use serenity::all::{
    AutoArchiveDuration, ChannelType, Context, CreateAttachment, CreateMessage, CreateThread,
    Message,
};

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

    let support_channel_ids = get_support_channel_ids(guild_id.get() as i64).await?;
    if !support_channel_ids.contains(&(msg.channel_id.get() as i64)) {
        return Ok(());
    }

    let guild_roles = guild_id.roles(&ctx).await?;

    let support_role_ids = get_support_role_ids(guild_id.get() as i64).await?;
    let support_role = guild_roles
        .into_iter()
        .find(|(role_id, _)| role_id.get() == (support_role_ids[0] as u64))
        .ok_or_else(|| Error::NoRole)?
        .1;

    if msg.member(&ctx).await?.roles.contains(&support_role.id) {
        return Ok(());
    }

    let attachments = get_attachments(msg).await?;

    let thread_id = get_support_thead_id(guild_id.get() as i64)
        .await
        .unwrap_or(0)
        + 1;
    update_support_thread_id(guild_id.get() as i64, thread_id).await?;

    let thread_name = get_thread_name(thread_id, &msg.author.name, &msg.content);

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

    msg.delete(&ctx).await?;

    Ok(())
}
