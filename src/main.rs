#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use ::serenity::Client;
use shuttle_runtime::SecretStore;

mod serenity;
mod webserver;

pub struct CombinedService {
    serenity_bot: Client,
    rocket_server: Rocket<Build>,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CombinedService {
    async fn bind(
        mut self,
        _addr: std::net::SocketAddr
    ) -> Result<(), shuttle_runtime::Error> {

        tokio::select!(
            _ = self.serenity_bot.start() => {},
            _ = self.rocket_server.launch() => {}
        );

        Ok(())
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> Result<CombinedService, shuttle_runtime::Error> {  
    let webserver = webserver::start().await;
    let discord = serenity::start(secrets).await;

    Ok(CombinedService {
        rocket_server: webserver,
        serenity_bot: discord
    })
}
