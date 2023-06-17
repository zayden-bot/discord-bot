mod commands {
    pub mod prefix_commands {
        pub mod gm;
        pub mod gn;
        pub mod ping;
    }
}
mod handler;

use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
