use std::io::prelude::*;
use dotenv;
mod events;
use events::Handler;
use serenity::client::Client;

#[tokio::main]
async fn main() {
    let token = dotenv::var("TWITTERBOT_TOKEN").expect("token");
    let mut client = Client::builder(token).event_handler(Handler).await
        .expect("Error creating client");
    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
