use std::env;
use rand::seq::SliceRandom;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use sqlx::postgres::PgPoolOptions;

async fn get_good_morning_images() -> Vec<String> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("Expected a database url in the environment"))
        .await
        .expect("Failed to connect to database");

    let rows = sqlx::query!("SELECT image_url FROM good_morning_images")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch good morning images");

    rows.iter().map(|row| row.image_url.clone()).collect()
}

pub async fn run(ctx: Context, msg: Message) {
    let mut good_morning_options = vec![
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

    let result = get_good_morning_images().await;

    good_morning_options.extend(result.iter().map(|s| s.as_str()));

    let good_night_message = match good_morning_options.choose(&mut rand::thread_rng()) {
        Some(message) => message,
        None => return,
    };

    if let Err(_) = msg.channel_id.say(&ctx, good_night_message).await {
        println!("Error sending message");
    }
}

