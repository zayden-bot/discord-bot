use std::fmt::Display;

use serenity::all::{
    ChannelId, Context, CreateMessage, DiscordJsonError, EditMessage, ErrorResponse, HttpError,
    Mentionable, UserId,
};

use crate::templates::{Template, TemplateInfo};

pub async fn update_embeds<T: Template>(
    ctx: &Context,
    row: &impl TemplateInfo,
    owner_name: &str,
    thread: impl Into<ChannelId>,
) {
    let thread = thread.into();

    let embed = T::thread_embed(row, owner_name);

    thread
        .edit_message(ctx, thread.get(), EditMessage::new().embed(embed))
        .await
        .unwrap();

    if let (Some(channel), Some(message)) = (row.alt_channel(), row.alt_message()) {
        let embed = T::message_embed(row, owner_name, thread);

        match channel
            .edit_message(ctx, message, EditMessage::new().embed(embed))
            .await
        {
            Ok(_)
            // Unknown Message
            | Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 10008, .. },
                ..
            }))) => {}
            Err(e) => panic!("{e:?}"),
        };
    }
}

pub enum Announcement {
    Joined { user: UserId, alternative: bool },
    Left(UserId),
}

impl Announcement {
    pub async fn send(&self, ctx: &Context, channel: ChannelId) {
        channel
            .send_message(ctx, CreateMessage::new().content(format!("{self}")))
            .await
            .unwrap();
    }
}

impl Display for Announcement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Announcement::Joined { user, alternative } if *alternative => {
                write!(f, "{} joined as an alternative", user.mention())
            }
            Announcement::Joined { user, .. } => {
                write!(f, "{} joined the fireteam", user.mention())
            }
            Announcement::Left(user) => write!(f, "{} left the fireteam", user.mention()),
        }
    }
}
