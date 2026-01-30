use crate::context::RunContext;
use anyhow::{Context as _, Result};
use chrono::Local;
use std::process::Command;

pub fn clone(ctx: &RunContext<'_>, git_uri: String, name: Option<String>) -> Result<()> {
    let dir_name = name.unwrap_or_else(|| generate_default_name(&git_uri));
    let target_path = ctx.prepare_target_path(&dir_name)?;
    let status = Command::new("git")
        .arg("clone")
        .arg(&git_uri)
        .arg(&target_path)
        .status()
        .with_context(|| format!("Failed to clone repository: {git_uri}"))?;
    if !status.success() {
        anyhow::bail!("git clone failed");
    }
    ctx.print_cd(&target_path);
    Ok(())
}

pub(crate) fn generate_default_name(git_uri: &str) -> String {
    let date = Local::now().format("%Y-%m-%d");
    let parts: Vec<&str> = git_uri.trim_end_matches(".git").split('/').collect();
    let (user, repo) = if parts.len() >= 2 {
        (parts[parts.len() - 2], parts[parts.len() - 1])
    } else {
        ("user", parts.last().copied().unwrap_or("repo"))
    };
    format!("{date}-{user}-{repo}")
}
