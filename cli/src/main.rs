use crate::commands::{
    add_command::{AddSchema, add_command},
    init_command::{InitSchema, init_command},
};
use cfg_if::cfg_if;
use clap::{ArgAction, Parser, Subcommand, ValueHint};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::error;
use reqwest::Client;
use std::{path::PathBuf, sync::LazyLock};
use thiserror::Error;

mod commands;
mod config;
mod preflights;
mod util;

cfg_if!(
    if #[cfg(windows)] {
        pub const NPM: &str = "npm.cmd";
    } else {
        pub const NPM: &str = "npm";
    }
);

#[derive(Error, Debug)]
pub enum MainError {
    #[error("Failed to init LogWrapper")]
    LogInit(#[from] log::SetLoggerError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    InitError(#[from] commands::init_command::InitError),

    #[error(transparent)]
    AddError(#[from] commands::add_command::AddError),
}

#[derive(Parser)]
#[command(name = "CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

pub static HTTPCLIENT: LazyLock<Client> = LazyLock::new(Client::new);

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Initializes lumina in your roblox-ts project and installing it's dependencies"
    )]
    Init {
        #[arg(value_hint = ValueHint::DirPath, default_value = ".", short, long, help = "Directory you want to init into")]
        cwd: PathBuf,
        #[arg(short, long, action = ArgAction::SetTrue, help = "Whether or not to skip prompt for confirmation in generating the components.json")]
        yes: bool,
        #[arg(short, long, action = ArgAction::SetTrue, help = "Whether or not to force initialization despite any checks")]
        force: bool,
        #[arg(short, long, action = ArgAction::SetTrue, help = "Whether or not to skip preflight checks and try and generate the components.json.")]
        skip_preflight: bool,
    },
    #[command(about = "Adds the desired component(s) to the project")]
    Add {
        #[arg(value_hint = ValueHint::DirPath, default_value = ".", short, long, help = "Directory you want to init into")]
        cwd: PathBuf,
        #[arg(
            short = 'C',
            long,
            help = "names or urls of components you want to add"
        )]
        components: Vec<String>,
    },
}

async fn run() -> Result<(), MainError> {
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();
    let level = logger.filter();
    let cli = Cli::parse();

    let mp = MultiProgress::new();

    LogWrapper::new(mp.clone(), logger).try_init()?;
    log::set_max_level(level);

    match &cli.command {
        Commands::Init {
            yes,
            force,
            cwd,
            skip_preflight,
        } => {
            init_command(
                &mp,
                InitSchema {
                    yes: *yes,
                    force: *force,
                    cwd: cwd.clone(),
                    skip_preflight: *skip_preflight,
                },
            )?;
        }
        Commands::Add { cwd, components } => {
            add_command(AddSchema {
                cwd: cwd.clone(),
                components: components.clone(),
            })
            .await?
        }
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(e) = run().await {
        error!("{}", e);
        std::process::exit(1);
    }
}
