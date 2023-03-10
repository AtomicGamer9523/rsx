mod routes;
use rsx::*;

const PORT: u16 = 8080;

async fn index(_req: tide::Request<()>) -> WebResult {
    routes::app::render()
        .into()
}

#[rsx::main]
async fn main() -> std::io::Result<()> {
    let mut app = rsx::newApp();
    app.at("/").get(index);
    app.listen(format!("[::]:{}", &PORT)).await?;
    Ok(())
}
