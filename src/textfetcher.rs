use std::ops::Not;

use serenity::{all::Reaction, all::Message, all::Ready, all::GetMessages, all::ChannelId, all::MessageId, all::ReactionType, all::Guild, all::ChannelType, all::Permissions, async_trait};
use serenity::prelude::*;
use serenity::http;
use tracing::{error, info};

pub struct TextFetcher;
#[async_trait]
impl EventHandler for TextFetcher {
    async fn reaction_add(&self, _: Context, react: Reaction) {
        info!("{} was reacted with!", react.emoji);
    }

    async fn reaction_remove(&self, _: Context, react: Reaction) {
        info!("message is connected!");
    }

    async fn message(&self, _: Context, message: Message ) {
        info!("{} is connected!", message.content);
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Ready event");
        info!("")
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        info!("Connected to server {}", guild.name);

        info!("Available emoji are: ");
        for (id, emoji) in guild.emojis.iter() {
            info!("Emoji {} with id {}", *emoji, *id)
        }

        info!("Available channels are: ");
        for (id, channel) in guild.channels {
            if channel.kind == ChannelType::Text {
                info!("Channel {} with id {}", channel.name, id)
            }
        }

        
        let channel_id = ChannelId::new(84969034449182720);
        let mut all_messages: Vec<Message> = Vec::new();
        let mut keep_scraping = true;
        let message_limit = 100;

        info!("Start scraping");
        let builder = GetMessages::new().limit(1);
        let first_message = channel_id.messages(ctx.http(), builder).await.unwrap();

        if first_message.is_empty() {
            info!("No messages in channel yet.");
        }
        else {
            let mut previous_message = first_message.iter().next().unwrap().id;
            all_messages.push(first_message.last().unwrap().clone());
            while keep_scraping {
                let builder = GetMessages::new().before(previous_message).limit(message_limit);
                let mut messages = channel_id.messages(ctx.http(), builder).await.unwrap();

                info!("Found {} messages.", messages.len());
                if messages.len() < message_limit.into() {
                    all_messages.append(&mut messages);
                    keep_scraping = false;
                    info!("Finished scraping, have {} messages.", all_messages.len());
                }
                else {
                    previous_message = messages.last().unwrap().id;
                    all_messages.append(&mut messages);
                    info!("Has a total of {} messages.", all_messages.len());
                }
            }
        }

        let mut best_messages: Vec<Message> = Vec::new();
        let mut highest_count: usize = 1;

        info!("Number of messages grasped: {}", all_messages.len());

        for message in all_messages {
            let current_count = message.reactions.len();
            if current_count == highest_count {
                best_messages.push(message);
            }
            else if current_count > highest_count {
                highest_count = current_count;
                best_messages.clear();
                best_messages.push(message);
            }
        }

        if best_messages.is_empty().not() {
            info!("The best messages of the channel are: ");
            // https://discord.com/channels/84969034449182720/84969034449182720/1202236367007129650
            //                              84969034449182720 84969034449182720 1202236367007129650
            for message in best_messages {
                let author_name = message.author.global_name.unwrap_or(message.author.name);
                let id: &str = &message.id.to_string();
                let url: String = "https://discord.com/channels/".to_string();
                let channel: &str = &"84969034449182720/";
                let url_to_post = url + channel + channel + id;
                info!("Message \"{}\" by {}, which has {} reactions, readable in context here: {}", message.content, author_name, highest_count, url_to_post);
                info!("The message had the following reacts: ");
                for reaction in message.reactions {
                    info!("{}", reaction.reaction_type)
                }
            }
        }
    }
}
