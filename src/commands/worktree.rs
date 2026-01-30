use crate::cli::ExecutionMode;
use crate::commands::{prepare_target_dir, print_cd_command, print_script_header};
use anyhow::{Context, Result};
use chrono::Local;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn worktree_dir(root: PathBuf, name: Option<String>, mode: ExecutionMode) -> Result<()> {
    let base_name = name.ok_or_else(|| anyhow::anyhow!("Name required for worktree"))?;
    let dir_name = format!("{}-{base_name}", Local::now().format("%Y-%m-%d"));
    let target_path = prepare_target_dir(&root, &dir_name)?;

    if !is_git_repo(&std::env::current_dir()?)? {
        anyhow::bail!("Not in a git repository");
    }

    match mode {
        ExecutionMode::Direct => {
            let status = Command::new("git")
                .arg("worktree")
                .arg("add")
                .arg(&target_path)
                .status()
                .with_context(|| "Failed to create git worktree")?;
            if !status.success() {
                anyhow::bail!("git worktree add failed");
            }
            print_cd_command(&target_path);
        }
        ExecutionMode::Script => {
            print_script_header();
            let abs = target_path
                .canonicalize()
                .unwrap_or_else(|_| target_path.clone());
            let abs_str = abs.to_string_lossy();
            println!("git worktree add '{abs_str}' && \\");
            println!("  cd '{abs_str}'");
        }
    }
    Ok(())
}

pub(crate) fn is_git_repo(path: &Path) -> Result<bool> {
    let git_dir = path.join(".git");
    Ok(git_dir.exists() && git_dir.is_dir())
}
