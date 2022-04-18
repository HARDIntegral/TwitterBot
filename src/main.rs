use std::io::prelude::*;
use std::env;
mod events;
use events::Handler;
use serenity::client::Client;
use serenity::prelude::GatewayIntents;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token, GatewayIntents::empty()).event_handler(Handler).await
        .expect("Error creating client");
    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
