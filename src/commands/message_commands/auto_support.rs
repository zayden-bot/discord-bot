use std::borrow::Cow;

use serenity::model::channel::{AttachmentType, Message};
use serenity::model::prelude::*;
use serenity::prelude::Context;
use crate::sqlx_lib::*;

fn get_welcome_message(support_role: &Role, user: &User) -> String {
    format!("{} {} wrote:", support_role, user)
}

#[allow(unused_qualifications)]
async fn get_attachments(msg: &Message) -> serenity::Result<Vec<AttachmentType>> {
    let mut attachments = Vec::new();
    for attachment in &msg.attachments {
        attachments.push(AttachmentType::Bytes {
            data: Cow::from(attachment.download().await?),
            filename: attachment.filename.clone(),
        });
    }
    Ok(attachments)
}

pub async fn run(ctx: &Context, msg: &Message) {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return,
    };

    let support_channel_ids = get_support_channel_ids(guild_id.0 as i64).await.unwrap();
    if !support_channel_ids.contains(&(msg.channel_id.0 as i64)) {
        return;
    }

    let attachments = get_attachments(msg).await.unwrap();

    let thread_id = match get_support_thead_id(guild_id.0 as i64).await {
        Ok(id) => {
            update_support_thread_id(guild_id.0 as i64, id + 1).await.unwrap();
            id + 1
        },
        Err(_) => {
            post_support_thread_id(guild_id.0 as i64, 1).await.unwrap();
            1
        }
    };

    let mut thread_name = format!("{} - {}", thread_id, msg.content);
    if thread_name.len() > 100 {
        thread_name = thread_name[..100].to_string();
    }

    let thread = msg
        .channel_id
        .create_private_thread(&ctx, |f| {
            f.name(thread_name);
            f.kind(ChannelType::PrivateThread);
            f.auto_archive_duration(10080);
            f
        })
        .await
        .unwrap();

    let support_role_ids = get_support_role_ids(msg.guild_id.unwrap().0 as i64).await.unwrap();
    let support_role = ctx
        .cache
        .role(guild_id, support_role_ids[0] as u64)
        .unwrap();

    thread
        .say(&ctx, get_welcome_message(&support_role, &msg.author))
        .await
        .unwrap();

    thread
        .send_files(&ctx, attachments, |m| m.content(&msg.content))
        .await
        .unwrap();

    match msg.delete(&ctx).await {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to delete message");
        }
    }
}
