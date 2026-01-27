use crate::cli::{ExecutionMode, TryArgs};
use anyhow::Result;
use std::path::PathBuf;

pub fn cd(root: PathBuf, query: Option<String>, args: &TryArgs, mode: ExecutionMode) -> Result<()> {
    if let Some(query) = &query {
        if looks_like_git_url(query) {
            return crate::commands::clone::clone(root, query.clone(), None, mode);
        }
    }

    if args.and_exit {
        eprintln!("Interactive selector (cancelled)");
        std::process::exit(1);
    }

    anyhow::bail!("Interactive selector not yet implemented")
}

pub fn looks_like_git_url(s: &str) -> bool {
    s.starts_with("http://")
        || s.starts_with("https://")
        || s.starts_with("git@")
        || s.starts_with("git://")
        || (s.contains("://") && s.contains(".git"))
}
