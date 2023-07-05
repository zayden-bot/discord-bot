use crate::chatgpt_lib;
use regex::Regex;
use serenity::model::channel::Message;
use serenity::model::prelude::*;
use serenity::prelude::*;

async fn get_display_name(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Option<String> {
    if let Some(member) = guild_id.member(ctx, &user_id).await.ok() {
        return Some(member.display_name().to_string());
    }
    None
}

fn parse_author_name(author_name: &str) -> Option<&str> {
    let regex_pattern = "^[a-zA-Z0-9_-]{1,64}$";

    let re = Regex::new(regex_pattern).unwrap();
    let matched_text = re.find(author_name);

    if let Some(m) = matched_text {
        return Some(m.as_str());
    }
    None
}

async fn parse_mentions(ctx: &Context, message: &Message) -> String {
    let mut parsed_content = message.content.clone();

    for mention in &message.mentions {
        let mention_tag = format!("<@{}>", mention.id);

        if mention.id == ctx.cache.current_user_id() {
            parsed_content = parsed_content.replace(&mention_tag, "");
            continue;
        }

        let guild_id = message.guild_id.unwrap();
        if let Some(name) = get_display_name(&ctx, guild_id, mention.id).await {
            parsed_content = parsed_content.replace(&mention_tag, &name);
        }
    }

    parsed_content
}

fn process_referenced_messages(ctx: &Context, msg: &Message) -> Vec<(bool, String)> {
    let mut contents = Vec::new();

    if let Some(referenced_message) = &msg.referenced_message {
        contents.push((
            &referenced_message.author.id == &ctx.cache.current_user_id(),
            referenced_message.content.to_string(),
        ));

        let nested_contents = process_referenced_messages(&ctx, &referenced_message);
        contents.extend(nested_contents);
    }

    contents
}

pub async fn run(ctx: &Context, msg: &Message) {
    if msg.content.ends_with("?") {
        if let Some(mention) = msg.mentions.first() {
            if mention.id == ctx.cache.current_user_id() {
                let parsed_message = parse_mentions(&ctx, &msg).await;

                let author_name = match parse_author_name(&msg.author.name) {
                    Some(name) => name,
                    None => {
                        msg.reply(&ctx, "Error: Invalid author name").await.unwrap();
                        return;
                    }
                };

                let replies = process_referenced_messages(&ctx, &msg);

                let response = match chatgpt_lib::chat(&parsed_message, author_name, replies).await
                {
                    Ok(response) => response,
                    Err(why) => {
                        msg.reply(&ctx, format!("Error: {}", why)).await.unwrap();
                        return;
                    }
                };

                msg.reply(&ctx, &response.choices[0].message.content)
                    .await
                    .unwrap();
            }
        }
    }
}
