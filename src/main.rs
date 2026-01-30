mod cli;
mod commands;
mod context;
mod runner;
use anyhow::Result;
use cli::Args;
use context::{RunContext, Runnable};

fn main() -> Result<()> {
    let args = Args::parse()?;
    let command = args.resolve_command();
    let root = args.root_path()?;
    let ctx = RunContext {
        root: root.clone(),
        args: &args,
    };
    command.run(&ctx)
}
