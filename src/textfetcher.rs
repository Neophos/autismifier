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

        let builder = GetMessages::new().limit(100);
        let messages = channel_id.messages(ctx.http(), builder).await.unwrap_or_default();

        let mut best_messages: Vec<Message> = Vec::new();
        let highest_count: usize = 0;

        info!("Number of messages grasped: {}", messages.len());

        for message in messages {
            let current_count = message.reactions.len();
            if current_count == highest_count {
                best_messages.push(message);
            }
            else if current_count > highest_count {
                best_messages.clear();
                best_messages.push(message);
            }
        }

        info!("The best messages of the channel are: ");
        for message in best_messages {
            info!("Message \"{}\" by {}, readable in context here: {}", message.content, message.author.name, message.id)
        }
    }
}
