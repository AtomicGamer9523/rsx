use actix_web::{get, App, HttpRequest};
#[path = "app.rsx"] mod app;
use rsx::*;

#[get("/")]
async fn index(_req: HttpRequest) -> HttpResult {
    rsx::res::Ok(
        app::render()
    )
}

#[rsx::main]
async fn main() -> std::io::Result<()> {
    WebServer::new(||{
        App::new()
        .service(index)
    })
    .https("127.0.0.1:3000")?
    .run().await
}