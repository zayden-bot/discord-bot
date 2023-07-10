use crate::sqlx_lib::get_good_night_images;
use rand::seq::SliceRandom;
use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn run(ctx: Context, msg: Message) {
    let mut good_night_options = vec![
        "I hope you have nightmares that keep you awake all night.",
        "Finally, a moment of peace without your annoying presence.",
        "Don't let the bedbugs bite, although they'd probably prefer you.",
        "I hope you toss and turn all night and wake up exhausted.",
        "I bet you'll still manage to find a way to ruin your own sleep.",
        "I hope you dream of all the failures in your life.",
        "It's the only time when the world is temporarily rid of you.",
        "May your dreams be filled with the embarrassment you deserve.",
        "I hope you wake up feeling just as miserable as you are.",
        "Finally, a chance for me to enjoy some peace and quiet.",
        "You and I both know it's way past when you actually want to sleep, and now you feel shitty about your choices, you worthless sack of meat",
    ];

    let result = get_good_night_images().await;

    good_night_options.extend(result.iter().map(|s| s.image_url.as_str()));

    let good_night_message = match good_night_options.choose(&mut rand::thread_rng()) {
        Some(message) => message,
        None => return,
    };

    if (msg.channel_id.say(&ctx, good_night_message).await).is_err() {
        println!("Error sending message");
    }
}
