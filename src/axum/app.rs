use std::{
    fs::{self, OpenOptions},
    future::Future,
    marker::PhantomData,
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{Error, Result};
use axum::Router;
use config::{builder::DefaultState, Config, ConfigBuilder, ConfigError, File};
use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{fmt::writer::MakeWriterExt, EnvFilter};

use super::server::{start_server, ServerConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub daemonize: bool,
    pub pid_file: Option<PathBuf>,
    pub enable_log: bool,
    pub log_file: Option<PathBuf>,
    pub log_level: String,
    pub rolling_log: RollingLog,
    pub stdout_file: Option<PathBuf>,
    pub stderr_file: Option<PathBuf>,
    #[serde(flatten)]
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RollingLog {
    Minutely,
    Hourly,
    Daily,
    Never,
}

impl Default for RollingLog {
    fn default() -> Self {
        RollingLog::Never
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            daemonize: false,
            pid_file: None,
            enable_log: true,
            log_file: None,
            log_level: "warn".into(),
            rolling_log: RollingLog::default(),
            stdout_file: None,
            stderr_file: None,
            server: ServerConfig::default(),
        }
    }
}

pub struct App<F, Fut> {
    config: Option<Config>,
    router_factory: F,
    _phantom: PhantomData<Fut>,
}

fn load_config_dir(
    path: &Path,
    mut config: ConfigBuilder<DefaultState>,
) -> ConfigBuilder<DefaultState> {
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("Read config dir error") {
            let entry_path = entry.expect("Read config dir error").path();
            if entry_path.is_dir() {
                config = load_config_dir(&entry_path, config);
            } else {
                config = config.add_source(File::from(entry_path));
            }
        }
    }

    config
}

impl<F, Fut> App<F, Fut>
where
    F: Fn(Config) -> Fut + Send + Clone + 'static,
    Fut: Future<Output = Result<Router>> + Send,
{
    pub fn new(factory: F) -> Self {
        App {
            config: None,
            router_factory: factory,
            _phantom: PhantomData,
        }
    }

    pub fn config_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        let mut config = Config::builder();
        if path.as_ref().is_file() {
            config = config.add_source(File::from(path.as_ref()));
        }

        if path.as_ref().is_dir() {
            config = load_config_dir(path.as_ref(), config);
        }

        self.config
            .replace(config.build().expect("Load configuration error"));
        self
    }

    pub fn run(self) -> Result<()> {
        let config = self.config.unwrap_or_default();
        let app_config = match config.get::<AppConfig>("application") {
            Ok(res) => res,
            Err(ConfigError::NotFound(_)) => AppConfig::default(),
            Err(err) => Err(Error::ConfigError(err))?,
        };

        enable_log(&app_config)?;
        daemonize(&app_config)?;

        let runtime = tokio::runtime::Runtime::new()?;

        runtime.block_on(async move {
            start_server(
                &app_config.server.listen,
                (self.router_factory.clone())(config.clone()).await?,
            )
            .await
        })
    }
}

fn enable_log(config: &AppConfig) -> Result<()> {
    if config.enable_log {
        if let Some(log_file) = &config.log_file {
            let path = log_file.as_path();
            let (directory, filename) = (
                path.parent().expect("'log_file' must be a file path."),
                path.file_name().expect("'log_file' must be a file path."),
            );

            if !directory.exists() {
                Err(Error::RuntimeBuildError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Log path is not exists.",
                )))?;
            }

            let rotation = match config.rolling_log {
                RollingLog::Daily => Rotation::DAILY,
                RollingLog::Hourly => Rotation::HOURLY,
                RollingLog::Minutely => Rotation::MINUTELY,
                RollingLog::Never => Rotation::NEVER,
            };
            let appender =
                tracing_appender::rolling::RollingFileAppender::new(rotation, directory, filename);

            let filter = EnvFilter::new(&config.log_level);

            tracing_subscriber::fmt()
                .with_ansi(false)
                .with_env_filter(filter)
                .with_writer(appender)
                .init();

            tracing::debug!("test log.");
        }
    }

    Ok(())
}

fn daemonize(config: &AppConfig) -> Result<()> {
    #[cfg(unix)]
    {
        use daemonize_me::Daemon;

        if config.daemonize {
            let mut daemon = Daemon::new();

            if let Some(pid_file) = config.pid_file {
                daemon = daemon.pid_file(pid_file, Some(false));
            }

            if let Some(stdout) = config.stdout_file {
                daemon = daemon.stdout(OpenOptions::new().create(true).append(true).open(stdout)?);
            }

            if let Some(stderr) = config.stderr_file {
                daemon = daemon.stderr(OpenOptions::new().create(true).append(true).open(stderr)?)
            }

            daemon.start().expect_err("Create daemonize process error");
        }

        Ok(())
    }

    #[cfg(not(unix))]
    {
        Ok(())
    }
}

use clap::{arg, Command};

pub fn run_standard_cli_app<F, Fut>(name: &str, app: App<F, Fut>) -> Result<()>
where
    F: Fn(Config) -> Fut + Send + Clone + 'static,
    Fut: Future<Output = Result<Router>> + Send,
{
    let matches = Command::new(name)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .about("Start application server.")
                .arg(arg!(config: -c --config <FILE> "Application server configuration file path (file or directory)."))
        )
        .get_matches();

    if let Some(start_command) = matches.subcommand_matches("start") {
        let config: PathBuf = start_command
            .value_of_t("config")
            .expect("Invalid configuration file path.");

        app.config_path(config).run()?;
    }

    Ok(())
}
