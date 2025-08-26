mod commands;
mod util;

use crate::commands::init_command::init_command;
use cfg_if::cfg_if;
use clap::Command;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::error;
use rust_embed::Embed;
use std::io::Error;
use thiserror::Error;

#[derive(Embed)]
#[folder = "assets"]
struct Asset;

#[derive(Error, Debug)]
pub enum MainError {
    #[error("Failed to init LogWrapper")]
    LogInit(#[from] log::SetLoggerError),

    #[error(transparent)]
    IoError(#[from] Error),

    #[error(transparent)]
    InputCommand(#[from] commands::init_command::InitError),
}

cfg_if!(
    if #[cfg(windows)] {
        pub const NPM: &str = "npm.cmd";
    } else {
        pub const NPM: &str = "npm";
    }
);

fn build_cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME")).subcommand(
        Command::new("init")
            .about("Initializes lumina in your roblox-ts project and installing it's dependencies"),
    )
}

fn main() -> Result<(), MainError> {
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();
    let level = logger.filter();

    let mp = MultiProgress::new();

    LogWrapper::new(mp.clone(), logger).try_init()?;
    log::set_max_level(level);

    let mut cmd = build_cli();
    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => init_command(&mp)?,
        _ => {
            cmd.print_help()?;
            error!("Exiting...")
        }
    }

    Ok(())
}
