use crate::commands::{add_command::add_command, init_command::init_command};
use cfg_if::cfg_if;
use clap::{Parser, Subcommand};
use include_dir::include_dir;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use thiserror::Error;

mod commands;
mod config;
mod util;

cfg_if!(
    if #[cfg(windows)] {
        pub const NPM: &str = "npm.cmd";
    } else {
        pub const NPM: &str = "npm";
    }
);

pub static COMPONENTS: include_dir::Dir<'_> = include_dir!("./components");
pub static ASSETS: include_dir::Dir<'_> = include_dir!("./cli/assets");
pub static LIBS: include_dir::Dir<'_> = include_dir!("./lib");
pub static STYLES: include_dir::Dir<'_> = include_dir!("./styles");

#[derive(Error, Debug)]
pub enum MainError {
    #[error("Failed to init LogWrapper")]
    LogInit(#[from] log::SetLoggerError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    InitError(#[from] commands::init_command::InitError),

    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    AddError(#[from] commands::add_command::AddError),
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
    Init,
    #[command(about = "Adds the desired component(s) to the project")]
    Add {
        #[arg(value_enum, required = true)]
        components: Vec<String>,
    },
}

fn main() -> Result<(), MainError> {
    config::init_config()?;

    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();
    let level = logger.filter();
    let cli = Cli::parse();

    let mp = MultiProgress::new();

    LogWrapper::new(mp.clone(), logger).try_init()?;
    log::set_max_level(level);

    match &cli.command {
        Commands::Init => {
            init_command(&mp)?;
        }
        Commands::Add { components } => add_command(components, &mp)?,
    }

    Ok(())
}
