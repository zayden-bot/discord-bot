use crate::chatgpt_lib;
use serenity::all::{parse_user_tag, Context, GuildId, Message, UserId};

async fn get_display_name(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Option<String> {
    if let Ok(member) = guild_id.member(ctx, &user_id).await {
        return Some(member.display_name().to_string());
    }
    None
}

async fn parse_mentions(ctx: &Context, message: &Message) -> String {
    let mut parsed_content = message.content.clone();

    for mention in &message.mentions {
        let mention_tag = format!("<@{}>", mention.id);

        if mention.id.get() == (ctx.shard_id.0 as u64) {
            parsed_content = parsed_content.replace(&mention_tag, "");
            continue;
        }

        let guild_id = message.guild_id.unwrap();
        if let Some(name) = get_display_name(ctx, guild_id, mention.id).await {
            parsed_content = parsed_content.replace(&mention_tag, &name);
        }
    }

    parsed_content
}

fn process_referenced_messages(ctx: &Context, msg: &Message) -> Vec<(bool, String)> {
    let mut contents = Vec::new();

    if let Some(referenced_message) = &msg.referenced_message {
        contents.push((
            referenced_message.author.id == (ctx.shard_id.0 as u64),
            referenced_message.content.to_string(),
        ));

        let nested_contents = process_referenced_messages(ctx, referenced_message);
        contents.extend(nested_contents);
    }

    contents
}

pub async fn run(ctx: &Context, msg: &Message) {
    // Check if message doesn't starts with ? and mentions the bot
    if !(msg.content.ends_with('?')
        && msg
            .mentions
            .iter()
            .any(|mention| mention.id == (ctx.shard_id.0 as u64)))
    {
        return;
    }

    let parsed_message = parse_mentions(ctx, msg).await;

    let author_name = match parse_user_tag(&msg.author.name) {
        Some(name) => name.0,
        None => &msg.author.name,
    };

    let replies = process_referenced_messages(ctx, msg);

    if let Ok(response) = chatgpt_lib::chat(&parsed_message, author_name, replies).await {
        msg.reply(&ctx, &response.choices[0].message.content)
            .await
            .unwrap();
    }
}
