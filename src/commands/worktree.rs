use crate::context::RunContext;
use anyhow::{Context as _, Result};
use chrono::Local;
use std::path::Path;
use std::process::Command;

pub fn worktree_dir(ctx: &RunContext, name: Option<String>) -> Result<()> {
    let base_name = name.ok_or_else(|| anyhow::anyhow!("Name required for worktree"))?;
    let dir_name = format!("{}-{base_name}", Local::now().format("%Y-%m-%d"));
    let target_path = ctx.prepare_target_path(&dir_name)?;

    if !is_git_repo(&std::env::current_dir()?)? {
        anyhow::bail!("Not in a git repository");
    }

    let status = Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg(&target_path)
        .status()
        .with_context(|| "Failed to create git worktree")?;
    if !status.success() {
        anyhow::bail!("git worktree add failed");
    }
    ctx.print_cd(&target_path);
    Ok(())
}

pub(crate) fn is_git_repo(path: &Path) -> Result<bool> {
    let git_dir = path.join(".git");
    Ok(git_dir.exists() && git_dir.is_dir())
}
