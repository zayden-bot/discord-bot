use std::borrow::Cow;
use serenity::model::channel::{Message, AttachmentType};
use serenity::model::prelude::*;
use serenity::prelude::Context;

fn get_welcome_message(support_role: Role, user: User) -> String {
    format!("{} {} wrote:", support_role, user)
}

async fn get_attachment_type_from_attachment(attachment: Attachment) -> AttachmentType<'static> {
    let attachment_type = AttachmentType::Bytes {
        data: Cow::from(attachment.download().await.unwrap()),
        filename: attachment.filename,
    };

    attachment_type
}

pub async fn run(ctx: Context, msg: Message) {
    const SUPPORT_CHANNEL_ID: u64 = 919950775134847016;
    const SUPPORT_ROLE_ID: u64 = 913374071239102504;

    if *(msg.channel_id.as_u64()) != SUPPORT_CHANNEL_ID {
        return;
    }

    let message_content = msg.content.clone();

    let mut thread_name = message_content.clone();
    if thread_name.len() > 100 {
        thread_name = thread_name[..100].to_string();
    }

    let mut attachments = Vec::new();

    for attachment in msg.attachments.clone() {
        attachments.push(get_attachment_type_from_attachment(attachment).await);
    }

    let thread = msg.channel_id.create_private_thread(&ctx, |f| {
        f.name(thread_name);
        f.kind(ChannelType::PrivateThread);
        f.auto_archive_duration(10080);
        f
    }).await.unwrap();

    let support_role = ctx.cache.role(msg.guild_id.unwrap(), SUPPORT_ROLE_ID).unwrap();

    thread.say(&ctx, get_welcome_message(support_role, msg.author.clone())).await.unwrap();

    thread.send_files(&ctx, attachments, |m| m.content(message_content)).await.unwrap();

    msg.delete(&ctx).await.unwrap();
}
