mod commands {
    pub mod message_commands {
        pub mod auto_support;
    }
    pub mod prefix_commands {
        pub mod gm;
        pub mod gn;
        pub mod ping;
    }
}
mod handler;
mod models;
mod sqlx_lib;

use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = &env::var("VIKTOR_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
