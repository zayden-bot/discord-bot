use serenity::all::{Context, Message};

pub async fn run(ctx: Context, msg: Message) {
    msg.channel_id
        .say(
            &ctx,
            "Mee6 is being deprecated, please use the `/rank` command instead",
        )
        .await
        .expect("Error sending message");
}
