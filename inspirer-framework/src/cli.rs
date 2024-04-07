use std::{env::current_dir, path::PathBuf, str::FromStr};

use clap::{Command, CommandFactory, FromArgMatches, Parser, Subcommand};
use daemonize::Daemonize;
use tokio::runtime::{self, Runtime};

use crate::{
    app::{create_app, AppTrait, Booter},
    command::CommandRegister,
    config::{config_keys, resolve_from_env, ConfigLoader, Environment},
    logger,
    server::start_server,
};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long, value_name = "DIR")]
    config: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    Start {
        #[arg(short, long)]
        daemonize: Option<bool>,
    },
}

pub fn run<T: AppTrait + 'static>() -> eyre::Result<()> {
    run_with_name_opt::<T>(None)
}

pub fn run_with_name<T: AppTrait + 'static>(name: &'static str) -> eyre::Result<()> {
    run_with_name_opt::<T>(Some(name))
}

pub fn run_with_name_opt<T: AppTrait + 'static>(name: Option<&'static str>) -> eyre::Result<()> {
    let mut subcommand_register = CommandRegister::new();
    T::commands(&mut subcommand_register);

    let cli = Subcommands::augment_subcommands(Cli::command())
        .subcommands(subcommand_register.commands.values().map(|(cmd, _)| cmd));

    let matches = cli.get_matches();
    let parsed_cli = Cli::from_arg_matches(&matches)?;
    let subcommands = Subcommands::from_arg_matches(&matches);

    // Load dotenv
    let _ = dotenvy::dotenv();

    // Load config
    let environment = resolve_from_env().into();
    let config = name
        .map_or(ConfigLoader::default(), ConfigLoader::with_name)
        .load_folder_opt(&environment, parsed_cli.config.as_ref().map(|f| f.as_path()))?;

    match &subcommands {
        Ok(std_subcommands) => {
            // Init log
            logger::init::<T>(config.get(config_keys::LOG)?);

            let task_span =
                tracing::span!(tracing::Level::DEBUG, "app", environment = %environment);
            let _guard = task_span.enter();

            match std_subcommands {
                Subcommands::Start { daemonize } => {
                    let daemonize = daemonize.unwrap_or(false);

                    if daemonize {
                        let d = Daemonize::new().working_directory(current_dir()?);
                        d.start()?;
                    }

                    create_tokio_runtime()?.block_on(async {
                        start_server::<T>(create_app::<T>(Booter::new(config)).await?).await
                    })?;
                }
            }
        }
        _ => match matches.subcommand() {
            Some((cmd, args)) => {
                if let Some((_, builder)) = subcommand_register.commands.remove(cmd) {
                    create_tokio_runtime()?.block_on(async {
                        builder(args)
                            .execute(create_app::<T>(Booter::new(config)).await?)
                            .await
                    })?;
                }
            }
            None => {}
        },
    }

    Ok(())
}

fn create_tokio_runtime() -> eyre::Result<Runtime> {
    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(Into::into)
}
