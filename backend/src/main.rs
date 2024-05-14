use backend::settings::get_settings;
use backend::app::Application;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let settings = get_settings().expect("Failed to read settings.");

    let application = Application::build(settings).await?;

    application.run().await?;

    Ok(())
}
