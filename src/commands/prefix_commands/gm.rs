use rand::seq::SliceRandom;
use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn run(ctx: Context, msg: Message) {
    let good_morning_messages = [
        "Wakey Wakey Fuck Facey",
        "Go back to fucking sleep!",
        "Is it a good morning or are you just trying to annoy me?",
        "Ugh, can't you just stay in bed and never wake up?",
        "More like a good opportunity for you to ruin my day.",
        "Well, it would be better if you weren't here.",
        "I didn't realize mornings could be so disappointing.",
        "Save your greetings for someone who actually cares.",
        "I guess that's your way of reminding me how miserable life is.",
        "I'm still not convinced it's a good one.",
        "Let's see how long your positivity lasts.",
        "How about you go back to sleep and never wake up again?",
    ];
    let good_morning_message = good_morning_messages.choose(&mut rand::thread_rng());

    msg.channel_id
        .say(&ctx.http, good_morning_message.unwrap())
        .await
        .expect("Error sending message");
}
