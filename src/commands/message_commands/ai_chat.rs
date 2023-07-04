use crate::chatgpt_lib;
use serenity::model::channel::Message;
use serenity::prelude::*;

async fn parse_mentions(ctx: &Context, message: &Message) -> String {
    let mut parsed_content = message.content.clone();

    for mention in &message.mentions {
        let guild_id = message.guild_id.unwrap();
        let member = guild_id.member(ctx, mention.id).await.ok();
        if let Some(member) = member {
            let nickname = member.display_name().to_string();
            let mention_tag = format!("<@{}>", mention.id);

            parsed_content = parsed_content.replace(&mention_tag, &nickname);
        }
    }

    parsed_content
}

pub async fn run(ctx: &Context, msg: &Message) {
    if msg.content.ends_with("?") {
        if let Some(mention) = msg.mentions.first() {
            if mention.id == ctx.cache.current_user_id() {
                let parsed_message = parse_mentions(&ctx, &msg).await;

                let response: chatgpt_lib::ChatResponse =
                    chatgpt_lib::chat(&parsed_message, &msg.author.name)
                        .await
                        .unwrap();

                msg.reply(&ctx, &response.choices[0].message.content)
                    .await
                    .unwrap();
            }
        }
    }
}
