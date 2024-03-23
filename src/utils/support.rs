use serenity::all::{Context, CreateMessage, GuildChannel, Mentionable, Role, User};

use crate::Result;

pub async fn send_support_message(
    ctx: &Context,
    thread: &GuildChannel,
    support_roles: &[&Role],
    author: &User,
    messages: Vec<CreateMessage>,
) -> Result<()> {
    let mentions: String = support_roles
        .iter()
        .map(|role| role.mention().to_string())
        .chain([author.mention().to_string()])
        .collect();

    thread.say(ctx, mentions).await?;

    for message in messages {
        thread.send_message(ctx, message).await?;
    }

    Ok(())
}

pub fn get_thread_name(thread_id: i32, author_name: &str, content: &str) -> String {
    format!("{} - {} - {}", thread_id, author_name, content)
        .chars()
        .take(100)
        .collect()
}
