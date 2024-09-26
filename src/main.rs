use anyhow::Context as _;
use serenity::{all::GatewayIntents, all::Client};
use shuttle_runtime::SecretStore;

pub mod bulkanalyser;
pub mod geoffposting;
pub mod reactcounter;
pub mod textfetcher;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets
    .get("DISCORD_TOKEN")
    .context("'DISCORD_TOKEN' was not found")?;

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::GUILD_EMOJIS_AND_STICKERS | GatewayIntents::GUILDS;

    let client = Client::builder(&token, intents)
        .event_handler(textfetcher::TextFetcher)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
