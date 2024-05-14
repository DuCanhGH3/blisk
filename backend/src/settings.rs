#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub protocol: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub debug: bool,
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

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let cwd = std::env::current_dir().expect("Failed to determine cwd");
    let settings_dir = cwd.join("settings");
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_owned())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_file = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(settings_dir.join("base.yaml")))
        .add_source(config::File::from(settings_dir.join(environment_file)))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    settings.try_deserialize::<Settings>()
}
