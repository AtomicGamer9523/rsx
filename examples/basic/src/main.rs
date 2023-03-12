use rsx::*;

const PORT: u16 = 8080;

route!("app");

async fn root(_req: tide::Request<()>) -> WebResult {
    app::render()
        .into()
}

#[rsx::main]
async fn main() -> std::io::Result<()> {
    let mut app = rsx::newApp();
    app.at("/").get(root);
    app.listen(format!("[::]:{}", &PORT)).await?;
    Ok(())
}