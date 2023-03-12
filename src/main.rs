use rsx::*;

const PORT: u16 = 8080;

route!("app");

async fn endpoint(_req: tide::Request<()>) -> WebResult {
    res::ok("Hello, world!")
        .into()
}

#[rsx::main]
async fn main() -> std::io::Result<()> {
    let mut app = rsx::newApp();
    app.at("/").get(endpoint);

    app.listen(format!("[::]:{}", &PORT)).await?;
    Ok(())
}