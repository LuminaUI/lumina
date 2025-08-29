use crate::commands::init_command::init_command;
use crate::schemas::InitSchema;
use cfg_if::cfg_if;
use clap::ArgAction;
use clap::ValueHint;
use clap::{Parser, Subcommand};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use std::path::PathBuf;
use thiserror::Error;

mod commands;
mod config;
mod errors;
mod preflights;
mod schemas;
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
}

#[derive(Parser)]
#[command(name = "CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Initializes lumina in your roblox-ts project and installing it's dependencies"
    )]
    Init {
        #[arg(value_hint = ValueHint::DirPath, default_value = ".", short, long)]
        cwd: PathBuf,
        #[arg(short, long, action = ArgAction::SetTrue)]
        yes: bool,
        #[arg(short, long, action = ArgAction::SetTrue)]
        force: bool,
        #[arg(short, long, action = ArgAction::SetTrue)]
        skip_preflight: bool,
    },
    #[command(about = "Adds the desired component(s) to the project")]
    Add {
        #[arg(value_enum, required = true)]
        components: Vec<String>,
    },
}

fn main() -> Result<(), MainError> {
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
        Commands::Add { components } => println!("Adding components {:?}", components),
    }

    Ok(())
}
