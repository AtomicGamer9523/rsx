use actix_web::{get, App, HttpRequest};
mod routes;
use rsx::*;


#[get("/")]
async fn index(_req: HttpRequest) -> HttpResult {
    rsx::res::Ok(
        routes::app::render()
    )
}


#[rsx::main]
async fn main() -> std::io::Result<()> {
    WebServer::new(||{
        App::new()
        .service(index)
    })
    .https("127.0.0.1:8080")?
    .run().await
}
