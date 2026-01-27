use crate::cli::ExecutionMode;
use crate::commands::{prepare_target_dir, print_cd_command, print_script_header};
use anyhow::{Context, Result};
use chrono::Local;
use std::path::PathBuf;
use std::process::Command;

pub fn clone(
    root: PathBuf,
    git_uri: String,
    name: Option<String>,
    mode: ExecutionMode,
) -> Result<()> {
    let dir_name = name.unwrap_or_else(|| generate_default_name(&git_uri));
    let target_path = prepare_target_dir(&root, &dir_name)?;

    match mode {
        ExecutionMode::Direct => {
            let status = Command::new("git")
                .arg("clone")
                .arg(&git_uri)
                .arg(&target_path)
                .status()
                .with_context(|| format!("Failed to clone repository: {git_uri}"))?;

            if !status.success() {
                anyhow::bail!("git clone failed");
            }

            print_cd_command(&target_path);
        }
        ExecutionMode::Script => {
            print_script_header();
            let abs_path = target_path
                .canonicalize()
                .unwrap_or_else(|_| target_path.clone());
            let abs_path_str = abs_path.to_string_lossy();
            println!("git clone '{git_uri}' '{abs_path_str}' && \\");
            println!("  cd '{abs_path_str}'");
        }
    }

    Ok(())
}

fn generate_default_name(git_uri: &str) -> String {
    let date = Local::now().format("%Y-%m-%d");

    let parts: Vec<&str> = git_uri.trim_end_matches(".git").split('/').collect();

    let (user, repo) = if parts.len() >= 2 {
        (parts[parts.len() - 2], parts[parts.len() - 1])
    } else {
        ("user", parts.last().copied().unwrap_or("repo"))
    };

    format!("{date}-{user}-{repo}")
}
