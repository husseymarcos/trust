use crate::cli::Command;
use crate::commands;
use crate::context::{RunContext, Runnable};
use anyhow::Result;

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
