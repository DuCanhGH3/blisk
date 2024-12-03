use blisk_backend::{app::Application, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    telemetry::init();

    let application = Application::build().await?;

    application.run().await?;

    Ok(())
}
