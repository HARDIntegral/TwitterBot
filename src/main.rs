use std::io::prelude::*;
use std::fs::File;
mod events;
use events::Handler;
use serenity::client::Client;


#[tokio::main]
async fn main() {
    // Getting token from private file
    let mut file = File::open("token.txt").expect("Unable to open token file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read token from file");
    let token: &str = &*contents;
    
    // Starting the bot
    let mut client = Client::builder(&token).event_handler(Handler).await
        .expect("Error crating client");
    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}