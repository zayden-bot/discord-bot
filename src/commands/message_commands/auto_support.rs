use crate::error;
use crate::sqlx_lib::{
    get_support_channel_ids, get_support_role_ids, get_support_thead_id, post_support_thread_id,
    update_support_thread_id,
};
use serenity::all::{
    AutoArchiveDuration, ChannelType, Context, CreateAttachment, CreateMessage, CreateThread,
    Message, Role, User,
};

fn get_welcome_message(support_role: &Role, user: &User) -> String {
    format!("{} {} wrote:", support_role, user)
}

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

pub async fn run(ctx: &Context, msg: &Message) -> error::Result<()> {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let support_channel_ids = get_support_channel_ids(guild_id.get() as i64)
        .await
        .unwrap();
    if !support_channel_ids.contains(&(msg.channel_id.get() as i64)) {
        return Ok(());
    }

    let guild_roles = ctx.http.get_guild_roles(guild_id).await.unwrap();

    let support_role_ids = get_support_role_ids(guild_id.get() as i64).await.unwrap();
    let support_role = guild_roles
        .into_iter()
        .find(|role| role.id.get() == (support_role_ids[0] as u64))
        .unwrap();

    if msg
        .member(&ctx)
        .await
        .unwrap()
        .roles
        .contains(&support_role.id)
    {
        return Ok(());
    }

    let attachments = get_attachments(msg).await.unwrap();

    let thread_id = match get_support_thead_id(guild_id.get() as i64).await {
        Ok(id) => {
            update_support_thread_id(guild_id.get() as i64, id + 1).await?;
            id + 1
        }
        Err(_) => {
            post_support_thread_id(guild_id.get() as i64, 1).await?;
            1
        }
    };

    let thread_name = format!("{} - {}", thread_id, msg.content)
        .chars()
        .take(100)
        .collect::<String>();

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
        .say(&ctx, get_welcome_message(&support_role, &msg.author))
        .await?;

    let chunks: Vec<String> = msg
        .content
        .chars()
        .collect::<Vec<char>>()
        .chunks(2000)
        .map(|c| c.iter().collect())
        .collect();

    for (index, chunk) in chunks.iter().enumerate() {
        if index == chunks.len() - 1 {
            thread
                .send_files(&ctx, attachments, CreateMessage::new().content(chunk))
                .await?;
            break;
        }

        thread.say(&ctx, chunk).await?;
    }

    msg.delete(&ctx).await?;

    Ok(())
}
