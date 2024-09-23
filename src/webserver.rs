use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub async fn start() -> Rocket<Build> {
    let webserver = rocket::build()
        .mount("/", routes![index]);

    return webserver
}