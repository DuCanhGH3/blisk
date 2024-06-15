use tracing_subscriber::layer::SubscriberExt;

use crate::settings::SETTINGS;

pub fn init() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        tracing_subscriber::EnvFilter::new({
            if SETTINGS.app.debug {
                "trace".to_string()
            } else {
                "info".to_string()
            }
        })
    });
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let json_log = if !SETTINGS.app.debug {
        let json_log = tracing_subscriber::fmt::layer().json();
        Some(json_log)
    } else {
        None
    };

    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(stdout_log)
        .with(json_log);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
}
