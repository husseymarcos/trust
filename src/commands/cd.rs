use crate::cli::{ExecutionMode, TryArgs};
use anyhow::Result;
use std::path::PathBuf;

const GIT_URL_PREFIXES: &[&str] = &["http://", "https://", "git@", "git://"];

pub fn cd(root: PathBuf, query: Option<String>, args: &TryArgs, mode: ExecutionMode) -> Result<()> {
    if let Some(q) = query.filter(|s| looks_like_git_url(s)) {
        return crate::commands::clone::clone(root, q, None, mode);
    }
    if args.and_exit {
        eprintln!("Interactive selector (cancelled)");
        std::process::exit(1);
    }
    anyhow::bail!("Interactive selector not yet implemented")
}

pub fn looks_like_git_url(s: &str) -> bool {
    GIT_URL_PREFIXES.iter().any(|p| s.starts_with(p)) || (s.contains("://") && s.contains(".git"))
}
