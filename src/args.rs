use crate::command::Command;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "trust", version = env!("CARGO_PKG_VERSION"), about = "Rusty directories for every vibe")]
#[command(
    long_about = "Quickly create and jump into fresh folders for your experiments."
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
    #[arg(
        long,
        value_name = "PATH",
        help = "Override tries directory (default: ~/src/tries)"
    )]
    pub path: Option<PathBuf>,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub query: Vec<String>,
}

impl Args {
    pub fn parse() -> Result<Self> {
        match Self::try_parse() {
            Ok(a) => Ok(a),
            Err(e) if e.kind() == clap::error::ErrorKind::DisplayHelp => {
                e.print().ok();
                std::process::exit(0);
            }
            Err(e) if e.kind() == clap::error::ErrorKind::DisplayVersion => {
                println!("try {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn resolve_command(&self) -> Command {
        self.command.clone().unwrap_or_else(|| Command::Cd {
            query: (!self.query.is_empty()).then(|| self.query.join(" ")),
        })
    }

    pub fn root_path(&self) -> Result<PathBuf> {
        if let Some(p) = &self.path {
            return Ok(p.clone());
        }
        if let Ok(p) = std::env::var("TRUST_PATH") {
            return Ok(PathBuf::from(p));
        }
        let home = std::env::var("HOME")
            .map_err(|_| anyhow::anyhow!("HOME environment variable not set"))?;
        Ok(PathBuf::from(home).join("src").join("tries"))
    }
}
