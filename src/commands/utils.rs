use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub fn prepare_target_dir(root: &PathBuf, dir_name: &str) -> Result<PathBuf> {
    let target_path = root.join(dir_name);
    if target_path.exists() {
        anyhow::bail!("Directory already exists: {}", target_path.display());
    }
    std::fs::create_dir_all(root)
        .with_context(|| format!("Failed to create root directory: {}", root.display()))?;
    Ok(target_path)
}

pub fn print_cd_command(path: &Path) {
    println!("cd '{}'", path.to_string_lossy());
}

pub fn print_script_header() {
    println!("# if you can read this, you didn't launch try from an alias. run try --help.");
}
