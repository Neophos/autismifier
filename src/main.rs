#[macro_use] extern crate rocket;
use shuttle_runtime::SecretStore;

mod serenity;
mod webserver;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore
) -> shuttle_rocket::ShuttleRocket {
    rocket::tokio::spawn(async {
        serenity::start(secrets).await.expect("Serenity bot failed");
    });

    // Start Rocket web server
    webserver::start().await
}
