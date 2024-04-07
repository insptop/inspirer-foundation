use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use tracing_subscriber::EnvFilter;

use crate::app::AppTrait;

// Function to initialize the logger based on the provided configuration
const MODULE_WHITELIST: &[&str] = &["loco_rs", "sea_orm_migration", "tower_http", "sqlx::query"];

// Define an enumeration for log levels
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub enum LogLevel {
    /// The "off" level.
    #[serde(rename = "off")]
    Off,
    /// The "trace" level.
    #[serde(rename = "trace")]
    Trace,
    /// The "debug" level.
    #[serde(rename = "debug")]
    Debug,
    /// The "info" level.
    #[serde(rename = "info")]
    #[default]
    Info,
    /// The "warn" level.
    #[serde(rename = "warn")]
    Warn,
    /// The "error" level.
    #[serde(rename = "error")]
    Error,
}

// Define an enumeration for log formats
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub enum Format {
    #[serde(rename = "compact")]
    #[default]
    Compact,
    #[serde(rename = "pretty")]
    Pretty,
    #[serde(rename = "json")]
    Json,
}

// Implement Display trait for LogLevel to enable pretty printing
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        to_variant_name(self).expect("only enum supported").fmt(f)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LoggerConfig {
    #[serde(default)]
    pub enable: bool,

    /// Enable nice display of backtraces, in development this should be on.
    /// Turn it off in performance sensitive production deployments.
    #[serde(default)]
    pub pretty_backtrace: bool,

    /// Set the logger level.
    ///
    /// * options: `trace` | `debug` | `info` | `warn` | `error`
    pub level: Option<LogLevel>,

    /// Set the logger format.
    ///
    /// * options: `compact` | `pretty` | `json`
    pub format: Option<Format>,

    /// Override our custom tracing filter.
    ///
    /// Set this to your own filter if you want to see traces from internal
    /// libraries. See more [here](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives)
    pub override_filter: Option<String>,
}

pub fn init<T: AppTrait + 'static>(config: LoggerConfig) {
    if !config.enable {
        return;
    }

    // tracing_subscriber::fmt()
    //     .pretty()
    //     .with_thread_names(true)
    //     .with_env_filter(EnvFilter::from_default_env())
    //     .init();

    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| {
            // user wanted a specific filter, don't care about our internal whitelist
            // or, if no override give them the default whitelisted filter (most common)
            config.override_filter.as_ref().map_or_else(
                || {
                    EnvFilter::try_new(
                        MODULE_WHITELIST
                            .iter()
                            .map(|m| format!("{}={}", m, config.level.unwrap_or_default()))
                            .chain(std::iter::once(format!(
                                "{}={}",
                                T::app_name(),
                                config.level.unwrap_or_default()
                            )))
                            .collect::<Vec<_>>()
                            .join(","),
                    )
                },
                EnvFilter::try_new,
            )
        })
        .expect("logger initialization failed");

    let builder = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_thread_names(true);

    match config.format.unwrap_or_default() {
        Format::Compact => builder.compact().init(),
        Format::Pretty => builder.pretty().init(),
        Format::Json => builder.json().init(),
    };
}
