use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "try")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Ephemeral workspace manager")]
#[command(
    long_about = "try is an ephemeral workspace manager that helps organize project directories with date-prefixed naming."
)]
pub struct TryArgs {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(
        long = "path",
        value_name = "PATH",
        help = "Override tries directory (default: ~/src/tries)"
    )]
    pub path: Option<PathBuf>,

    #[arg(long = "no-colors", help = "Disable ANSI color codes in output")]
    pub no_colors: bool,

    #[arg(
        long = "and-exit",
        hide = true,
        help = "Test mode: render TUI once and exit"
    )]
    pub and_exit: bool,

    #[arg(
        long = "and-keys",
        hide = true,
        help = "Test mode: inject key sequence"
    )]
    pub and_keys: Option<String>,

    #[arg(
        long = "no-expand-tokens",
        hide = true,
        help = "Test mode: output raw tokens"
    )]
    pub no_expand_tokens: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub query: Vec<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    #[command(about = "Output shell function definition for shell integration")]
    Init {
        #[arg(value_name = "PATH", help = "Override default tries directory")]
        path: Option<PathBuf>,
    },
    #[command(about = "Interactive directory selector with fuzzy search")]
    Cd {
        #[arg(value_name = "QUERY", help = "Initial filter text for fuzzy search")]
        query: Option<String>,
    },
    #[command(about = "Execute command and output shell script")]
    Exec {
        #[command(subcommand)]
        exec_command: ExecCommand,
    },
    #[command(about = "Clone a git repository into a dated directory")]
    Clone {
        #[arg(value_name = "URL", help = "Git repository URL")]
        url: String,
        #[arg(value_name = "NAME", help = "Custom name suffix")]
        name: Option<String>,
    },
    #[command(about = "Create a git worktree in a dated directory")]
    Worktree {
        #[arg(value_name = "NAME", help = "Branch or worktree name")]
        name: String,
    },
    #[command(name = ".", about = "Shorthand for worktree (requires name)")]
    Dot {
        #[arg(value_name = "NAME", help = "Branch or worktree name")]
        name: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum ExecCommand {
    Cd {
        #[arg(value_name = "QUERY")]
        query: Option<String>,
    },
    Clone {
        #[arg(value_name = "URL")]
        url: String,
        #[arg(value_name = "NAME")]
        name: Option<String>,
    },
    Worktree {
        #[arg(value_name = "NAME")]
        name: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionMode {
    Direct,
    Script,
}

impl TryArgs {
    pub fn root_path(&self) -> Result<PathBuf> {
        if let Some(path) = &self.path {
            return Ok(path.clone());
        }

        if let Ok(path) = std::env::var("TRY_PATH") {
            return Ok(PathBuf::from(path));
        }

        let home = std::env::var("HOME")
            .map_err(|_| anyhow::anyhow!("HOME environment variable not set"))?;
        Ok(PathBuf::from(home).join("src").join("tries"))
    }
}
