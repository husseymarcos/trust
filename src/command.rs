use crate::commands;
use crate::context::RunContext;
use anyhow::Result;
use clap::Subcommand;
use std::path::PathBuf;



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

pub trait Runnable {
    fn run(self, ctx: &RunContext<'_>) -> Result<()>;
}

impl Runnable for Command {
    fn run(self, ctx: &RunContext<'_>) -> Result<()> {
        match self {
            Command::Init { path } => commands::init::init(path),
            Command::Cd { query } => commands::cd::cd(ctx, query),
            Command::Clone { url, name } => commands::clone::clone(ctx, url, name),
            Command::Worktree { name } | Command::Dot { name } => {
                commands::worktree::worktree_dir(ctx, Some(name))
            }
        }
    }
}
