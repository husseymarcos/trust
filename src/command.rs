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
