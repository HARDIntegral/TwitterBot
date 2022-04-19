use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::Message;
use serenity::model::channel::ChannelType;
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
        if msg.author.id == CLIENT_ID {
            return;
        }

        const CLIENT_ID: u64 = 965142516141789216;
        const CHANNEL_ID: u64 = 965109206468395049;
        let mut is_sus: bool = false;
        let mut is_dms: bool = false;
        
        if msg.channel_id == CHANNEL_ID {
            is_sus = true;
        } 
        let channel = ChannelId(*msg.channel_id.as_u64()).to_channel(&context.http).await.unwrap();
        match channel.private() {
            Some(private) => { 
                is_sus = true; 
                is_dms = true;
            },
            None => {},
        }

        if is_sus {
            if !is_dms {
                msg.delete(&context).await;
            }
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(err) => {
                    println!("Error getting channel: {:?}", err);
                    return;
                },
            };
            let msg_content: String = msg.content.clone();
            let rand_id: i32 = rand::thread_rng().gen_range(100000,999999);
            let mut return_msg = MessageBuilder::new()
                .push("[")
                .push(&rand_id)
                .push("] ")
                .push(msg_content)
                .build();
            if let Err(err) = ChannelId(CHANNEL_ID).say(&context.http, &return_msg).await {
                println!("Error sending message: {:?}", err);
                return;
            }
            for i in &msg.attachments {
                ChannelId(CHANNEL_ID).say(&context.http, &i.url).await;
            }
        }
    }
}