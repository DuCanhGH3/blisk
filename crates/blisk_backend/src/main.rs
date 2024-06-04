use blisk_backend::app::Application;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let application = Application::build().await?;

    application.run().await?;

    Ok(())
}
