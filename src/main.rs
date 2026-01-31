mod args;
mod command;
mod commands;
mod context;
mod tui;
use anyhow::Result;
use args::Args;
use command::Runnable;
use context::RunContext;

fn main() -> Result<()> {
    let args = Args::parse()?;
    let command = args.resolve_command();
    let root = args.root_path()?;
    let ctx = RunContext { root: root.clone() };
    command.run(&ctx)
}
