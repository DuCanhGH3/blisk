use crate::{routes, settings::Settings};
use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct ApplicationState {
    pub pool: Pool<Postgres>,
}

pub struct Application {
    port: u16,
    server: Serve<Router, Router>,
}

impl Application {
    pub async fn build(settings: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        let pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(settings.database.to_pg_connect_options());

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to migrate database.");

        let app_state = ApplicationState { pool };
        let listener = tokio::net::TcpListener::bind(&address).await?;
        let port = listener.local_addr().unwrap().port();
        let app = Router::new()
            .route("/health", get(routes::health::health_check))
            .route("/users/register", post(routes::users::register))
            .with_state(app_state);
        let server = axum::serve(listener, app);

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
