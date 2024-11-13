use serenity::all::{Context, Message};

use crate::chatgpt_lib;
use crate::Error;
use crate::Result;

async fn parse_mentions(ctx: &Context, message: &Message) -> Result<String> {
    let mut parsed_content = message.content.clone();

    for mention in &message.mentions {
        let mention_tag = format!("<@{}>", mention.id);

        if mention.id.get() == (ctx.shard_id.0 as u64) {
            parsed_content = parsed_content.replace(&mention_tag, "");
            continue;
        }

        let guild_id = message.guild_id.ok_or_else(|| Error::NotInGuild)?;
        let member = guild_id.member(&ctx, mention.id).await?;

        parsed_content = parsed_content.replace(&mention_tag, member.display_name());
    }

    Ok(parsed_content)
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

pub async fn run(ctx: &Context, msg: &Message) -> Result<()> {
    // Check if message doesn't start with ? and mentions the zayden-bot
    if !(msg.content.ends_with('?')
        && msg
            .mentions
            .iter()
            .any(|mention| mention.id == (ctx.shard_id.0 as u64)))
    {
        return Ok(());
    }

    let parsed_message = parse_mentions(ctx, msg).await?;
    let replies = process_referenced_messages(ctx, msg);

    let response = chatgpt_lib::chat(&parsed_message, &msg.author.name, replies).await?;
    msg.reply(&ctx, &response.choices[0].message.content)
        .await?;

    Ok(())
}
