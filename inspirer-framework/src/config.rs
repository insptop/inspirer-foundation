use std::{
    env, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use config::{Config as Cfg, File, FileFormat};
use once_cell::sync::Lazy;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::{Error, Result};

static DEFAULT_FOLDER: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("config"));
pub const DEFAULT_ENVIRONMENT: &str = "development";
pub const INSPIRER_ENV: &str = "INSPIRER_ENV";

pub(crate) mod config_keys {
    pub const LOG: &'static str = "log";
    pub const SERVER: &'static str = "server";
}

#[derive(Debug, Clone)]
pub enum Environment {
    Production,
    Development,
    Test,
    Any(String),
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Development => write!(f, "development"),
            Self::Production => write!(f, "production"),
            Self::Test => write!(f, "test"),
            Self::Any(s) => s.fmt(f),
        }
    }
}

impl FromStr for Environment {
    type Err = &'static str;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "production" => Ok(Self::Production),
            "development" => Ok(Self::Development),
            "test" => Ok(Self::Test),
            s => Ok(Self::Any(s.to_string())),
        }
    }
}

impl From<String> for Environment {
    fn from(env: String) -> Self {
        Self::from_str(&env).unwrap_or(Self::Any(env))
    }
}

pub fn resolve_from_env() -> String {
    std::env::var(INSPIRER_ENV).unwrap_or_else(|_| DEFAULT_ENVIRONMENT.to_string())
}

#[derive(Clone)]
pub struct Config {
    config: Cfg,
}

impl Config {
    pub fn get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<T> {
        self.config.get(key).map_err(Into::into)
    }
}

#[derive(Default)]
pub struct ConfigLoader {
    name: Option<&'static str>,
}

impl ConfigLoader {
    pub fn with_name(name: &'static str) -> Self {
        ConfigLoader { name: Some(name) }
    }

    pub fn load(&self, env: &Environment) -> Result<Config> {
        self.load_folder(env, &DEFAULT_FOLDER)
    }

    pub fn load_folder_opt(&self, env: &Environment, folder: Option<&Path>) -> Result<Config> {
        if let Some(folder) = folder {
            self.load_folder(env, folder)
        } else {
            self.load(env)
        }
    }

    pub fn load_folder(&self, env: &Environment, folder: &Path) -> Result<Config> {
        let files = if let Some(name) = self.name {
            [
                folder.join(name).join(format!("{env}.local.toml")),
                folder.join(name).join(format!("{env}.toml")),
            ]
        } else {
            [
                folder.join(format!("{env}.local.toml")),
                folder.join(format!("{env}.toml")),
            ]
        };

        let selected_path = files
            .iter()
            .find(|p| p.exists())
            .ok_or_else(|| Error::Message("no configuration file found".to_string()))?;

        tracing::info!(selected_path =? selected_path, "loading environment from");

        Self::load_config(&selected_path)
    }

    fn load_config(config_file: &Path) -> Result<Config> {
        let mut context = Context::new();
        for (key, val) in env::vars() {
            context.insert(key, &val);
        }

        let temp = fs::read_to_string(config_file)?;
        let config_content = Tera::one_off(&temp, &context, false)?;

        let config = Cfg::builder()
            .add_source(File::from_str(&config_content, FileFormat::Toml))
            .build()?;

        Ok(Config { config })
    }
}
