use std::str::FromStr;

use crate::{routes, settings::SETTINGS, utils::constants::UPLOADS_DIRECTORY};
use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderName, HeaderValue, Method},
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir, set_header::SetResponseHeaderLayer};

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
        tokio::fs::create_dir_all(UPLOADS_DIRECTORY)
            .await
            .expect("Failed to create `uploads` directory");

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
        let cors = CorsLayer::new()
            .allow_credentials(true)
            .allow_headers([header::ACCEPT, header::CONTENT_TYPE, header::RANGE])
            .allow_methods(vec![
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_origin(
                SETTINGS
                    .frontend
                    .url
                    .parse::<HeaderValue>()
                    .expect("Failed to parse frontend URI."),
            );
        let robots = SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-robots-tag"),
            HeaderValue::from_static("noindex"),
        );
        let app = Router::new()
            .route("/health", get(routes::health::health_check))
            .route(
                "/books",
                post(routes::books::create).get(routes::books::read),
            )
            .route("/books/:slug", get(routes::books::read_slug))
            .route("/books/categories", get(routes::books::read_categories))
            .route("/books/metadata", get(routes::books::read_metadata))
            .route(
                "/posts",
                post(routes::posts::create)
                    .get(routes::posts::read)
                    .patch(routes::posts::update)
                    .delete(routes::posts::delete),
            )
            .route("/posts/:id", get(routes::posts::read_slug))
            .route(
                "/comments",
                post(routes::comments::create)
                    .get(routes::comments::read)
                    .patch(routes::comments::update)
                    .delete(routes::comments::delete),
            )
            .route("/comments/replies", get(routes::comments::read_replies))
            .route(
                "/reactions",
                post(routes::reactions::create).delete(routes::reactions::delete),
            )
            .route("/auth/authenticate", post(routes::auth::authenticate))
            .route("/auth/confirm", post(routes::auth::confirm))
            .route("/auth/register", post(routes::auth::register))
            .route("/auth/login", post(routes::auth::login))
            .route("/assets/upload", post(routes::files::upload))
            .nest_service("/assets", ServeDir::new(UPLOADS_DIRECTORY))
            .with_state(app_state)
            .layer(ServiceBuilder::new().layer(cors).layer(robots))
            .layer(DefaultBodyLimit::max(10_000_000));
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
