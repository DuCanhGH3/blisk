use std::sync::LazyLock;

#[derive(serde::Deserialize, Clone)]
pub struct AppSettings {
    /// Whether the application should run in debug mode.
    pub debug: bool,
    /// The port the application should listen on.
    pub port: u16,
    /// The hostname the application should listen on.
    pub host: String,
    /// The public address that should point to the backend.
    pub base: String,
    /// The protocol the application should use.
    pub protocol: String,
}
#[derive(serde::Deserialize, Clone)]
pub struct EmailHostSettings {
    pub name: String,
    pub password: String,
}
#[derive(serde::Deserialize, Clone)]
pub struct EmailSettings {
    pub host: EmailHostSettings,
}
#[derive(serde::Deserialize, Clone)]
pub struct FrontendSettings {
    /// The URL to the frontend.
    pub url: String,
}
#[derive(serde::Deserialize, Clone)]
pub struct RedisSettings {
    pub uri: String,
}
#[derive(serde::Deserialize, Clone)]
pub struct SecretSettings {
    /// The HMAC secret used for issuing tokens.
    pub sec: String,
    /// A token's expiration time in seconds.
    pub exp: i64,
}
#[derive(serde::Deserialize, Clone)]
pub struct AuthSettings {
    /// A refresh token's expiration time in seconds.
    pub refresh: SecretSettings,
    /// The HMAC secret used for issuing refresh tokens.
    pub access: SecretSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    /// Application-related settings.
    pub app: AppSettings,
    /// Email-related settings.
    pub email: EmailSettings,
    /// Frontend-related settings.
    pub frontend: FrontendSettings,
    /// Redis-related settings.
    pub redis: RedisSettings,
    /// Secret-related settings.
    pub secret: SecretSettings,
    /// Authencation-related setttings.
    pub auth: AuthSettings,
}

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> String {
        match self {
            Environment::Development => "development".to_owned(),
            Environment::Production => "production".to_owned(),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            other => Err(format!("{} is not a supported environment", other)),
        }
    }
}

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    let cwd = std::env::current_dir().expect("Failed to determine cwd");
    let settings_dir = cwd.join("settings");
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_owned())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    println!("Starting app in {} mode.", environment.as_str());

    let environment_file = format!("{}.yaml", environment.as_str());

    let config = config::Config::builder()
        .add_source(config::File::from(settings_dir.join("base.yaml")))
        .add_source(config::File::from(settings_dir.join(environment_file)))
        .add_source(config::Environment::with_prefix("BLISK").separator("_"))
        .build()
        .expect("Failed to build configuration");

    config
        .try_deserialize::<Settings>()
        .expect("Failed to parse settings")
});
