use crate::args::Args;
use anyhow::{Context as _, Result};
use std::path::{Path, PathBuf};

pub struct RunContext<'a> {
    pub root: PathBuf,
    pub args: &'a Args,
}

impl RunContext<'_> {
    pub fn prepare_target_path(&self, dir_name: &str) -> Result<PathBuf> {
        let target_path = self.root.join(dir_name);
        if target_path.exists() {
            anyhow::bail!("Directory already exists: {}", target_path.display());
        }
        std::fs::create_dir_all(&self.root)
            .with_context(|| format!("Failed to create root directory: {}", self.root.display()))?;
        Ok(target_path)
    }

    pub fn print_cd(&self, path: &Path) {
        println!("cd '{}'", path.to_string_lossy());
    }
}

pub trait Runnable {
    fn run(self, ctx: &RunContext<'_>) -> anyhow::Result<()>;
}
