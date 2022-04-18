use serenity::{
    async_trait,
    prelude::*,
    model::{channel::Message, gateway::Ready},
    utils::MessageBuilder,
};
use rand::Rng;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }

    async fn message(&self, context: Context, msg: Message) {
        if msg.author.id == 965142516141789216 {
            return;
        }
        if msg.channel_id == 965109206468395049 {
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
            
            if let Err(err) = msg.channel_id.say(&context.http, &return_msg).await {
                println!("Error sending message: {:?}", err);
                return;
            }
        }
    }
}