use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn run(ctx: Context, msg: Message) {
    msg.channel_id
        .say(&ctx, "Pong!")
        .await
        .expect("Error sending message");
}
