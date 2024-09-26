use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};

struct Bot;
#[async_trait]
impl EventHandler for Bot {
    // async fn message(&self, ctx: Context, msg: Message) {
    //     if msg.content == "!hello" {
    //         if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
    //             error!("Error sending message: {:?}", e);
    //         }
    //     }
    // }
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
