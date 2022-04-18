use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::{
    ApplicationCommand,
    ApplicationCommandInteractionDataOptionValue,
    ApplicationCommandOptionType,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::model::prelude::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use rand::Rng;
use std::env;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );
        let commands = GuildId::set_application_commands(&guild_id, &context.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("susify").description("Messages sent to channel id given are anonymous")
                        .create_option(|option| {
                            option
                                .name("channel id")
                                .kind(ApplicationCommandOptionType::Integer)
                                .required(true)        
                        })
                }).create_application_command(|command| {
                    command.name("desusify").description("Messages sent to channel id given are no longer anonymous")
                        .create_option(|option| {
                            option
                                .name("channel id")
                                .kind(ApplicationCommandOptionType::Integer)
                                .required(true)
                        })
                })
        }).await;
    }

    async fn message(&self, context: Context, msg: Message) {
        if msg.author.id == 965605739760615474 {
            return;
        }
        if msg.channel_id == 965096943153971230 {
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
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "susify" => {"Channel added to the sus list!".to_string()},
                "desusify" => {"Channel removed from the sus list!".to_string()},
                _ => "Commands not implemented".to_string()
            };
            if let Err(err) = command.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            }).await {
                println!("Cannot respond to slash command: {}", err);
            }
        }
    }
}
