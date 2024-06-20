use std::str::FromStr;

use crate::{routes, settings::SETTINGS};
use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub redis_client: redis::Client,
}

pub struct Application {
    port: u16,
    server: Serve<Router, Router>,
}

impl Application {
    pub async fn build() -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", SETTINGS.app.host, SETTINGS.app.port);

        let db_uri = std::env::var("DATABASE_URL").expect("Failed to read database URI");

        let pool_options =
            PgConnectOptions::from_str(&db_uri).expect("Failed to parse database URI");

        let pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(pool_options);

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to migrate database");

        let redis_client = redis::Client::open(SETTINGS.redis.uri.as_str())
            .expect("Failed to create a Redis client");

        let app_state = AppState { pool, redis_client };
        let listener = tokio::net::TcpListener::bind(&address).await?;
        let port = listener.local_addr().unwrap().port();
        let app = Router::new()
            .route("/health", get(routes::health::health_check))
            .route("/books/create", post(routes::books::create))
            .route("/books/read", get(routes::books::read))
            .route("/posts/create", post(routes::posts::create))
            .route("/posts/read", get(routes::posts::read))
            .route("/comments/create", post(routes::comments::create))
            .route("/comments/read", get(routes::comments::read))
            .route("/reactions/create", post(routes::reactions::create))
            .route("/users/authenticate", post(routes::users::authenticate))
            .route("/users/confirm", post(routes::users::confirm))
            .route("/users/login", post(routes::users::login))
            .route("/users/read", get(routes::users::read))
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
