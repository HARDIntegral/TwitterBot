use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use  serenity::model::id::ChannelId;
use rand::Rng;
use std::env;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    async fn ready(&self, context: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
        
    }

    async fn message(&self, context: Context, msg: Message) {
        const CLIENT_ID: u64 = 965142516141789216;
        const CHANNEL_ID: u64 = 965109206468395049;
        const DM_ID: u64 = 965749825444397067;
        if msg.author.id == CLIENT_ID {
            return;
        }
        if msg.channel_id == CHANNEL_ID || msg.channel_id == DM_ID {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(err) => {
                    println!("Error getting channel: {:?}", err);
                    return;
                },
            };
            let msg_content: String = msg.content.clone();
            msg.delete(&context).await;
            let rand_id: i32 = rand::thread_rng().gen_range(100000,999999);
            let return_msg = MessageBuilder::new()
                .push("[")
                .push(&rand_id)
                .push("] ")
                .push(msg_content)
                .build();
            
            if let Err(err) = ChannelId(CHANNEL_ID).say(&context.http, &return_msg).await {
                println!("Error sending message: {:?}", err);
                return;
            }
        }
    }
}
