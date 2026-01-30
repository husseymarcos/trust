use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "try", version = env!("CARGO_PKG_VERSION"), about = "Ephemeral workspace manager")]
#[command(
    long_about = "Trust is an ephemeral workspace manager that helps organize project directories with date-prefixed naming."
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
    #[arg(long, help = "Disable ANSI color codes in output")]
    pub no_colors: bool,
    #[arg(long, hide = true)]
    pub and_exit: bool,
    #[arg(long, hide = true)]
    pub and_keys: Option<String>,
    #[arg(long, hide = true)]
    pub no_expand_tokens: bool,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub query: Vec<String>,
}

/// Top-level commands: what the user invokes directly (`trust init`, `trust clone …`, etc.).
#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    #[command(about = "Output shell function definition for shell integration")]
    Init { path: Option<PathBuf> },
    #[command(about = "Interactive directory selector with fuzzy search")]
    Cd { query: Option<String> },
    #[command(about = "Clone a git repository into a dated directory")]
    Clone { url: String, name: Option<String> },
    #[command(about = "Create a git worktree in a dated directory")]
    Worktree { name: String },
    #[command(name = ".", about = "Shorthand for worktree (requires name)")]
    Dot { name: String },
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
        if let Ok(p) = std::env::var("TRY_PATH") {
            return Ok(PathBuf::from(p));
        }
        let home = std::env::var("HOME")
            .map_err(|_| anyhow::anyhow!("HOME environment variable not set"))?;
        Ok(PathBuf::from(home).join("src").join("tries"))
    }
}
