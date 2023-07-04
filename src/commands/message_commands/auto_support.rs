use std::borrow::Cow;

use serenity::model::channel::{AttachmentType, Message};
use serenity::model::prelude::*;
use serenity::prelude::Context;
use crate::sqlx_lib::{get_support_thead_id, post_support_thread_id, update_support_thread_id};

fn get_welcome_message(support_role: &Role, user: &User) -> String {
    format!("{} {} wrote:", support_role, user)
}

async fn get_attachment_type_from_attachment(attachment: &Attachment) -> AttachmentType<'static> {
    let attachment_type = AttachmentType::Bytes {
        data: Cow::from(attachment.download().await.unwrap()),
        filename: attachment.filename.clone(),
    };

    attachment_type
}

pub async fn run(ctx: &Context, msg: &Message) {
    const SUPPORT_CHANNEL_ID: u64 = 919950775134847016;
    const SUPPORT_ROLE_ID: u64 = 913374071239102504;

    if msg.channel_id.0 != SUPPORT_CHANNEL_ID {
        return;
    }

    let guild_id = msg.guild_id.unwrap();

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

    let mut attachments = Vec::new();

    for attachment in &msg.attachments {
        attachments.push(get_attachment_type_from_attachment(&attachment).await);
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

    let support_role = ctx
        .cache
        .role(&guild_id, SUPPORT_ROLE_ID)
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
