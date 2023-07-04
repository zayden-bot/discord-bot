use crate::chatgpt_lib;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn run(ctx: &Context, msg: &Message) {
    if msg.content.ends_with("?") {
        if let Some(mention) = msg.mentions.first() {
            if mention.id == ctx.cache.current_user_id() {
                let response: chatgpt_lib::ChatResponse =
                    chatgpt_lib::chat(&msg.content, &msg.author.name)
                        .await
                        .unwrap();

                msg.reply(&ctx, &response.choices[0].message.content)
                    .await
                    .unwrap();
            }
        }
    }
}
