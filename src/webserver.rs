#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub async fn start() -> shuttle_rocket::ShuttleRocket {
    let webserver = rocket::build()
        .mount("/", routes![index]);

    Ok(webserver.into())
}