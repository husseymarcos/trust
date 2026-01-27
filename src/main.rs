mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;
use cli::TryArgs;

fn main() -> Result<()> {
    let args = match TryArgs::try_parse() {
        Ok(args) => args,
        Err(e) if e.kind() == clap::error::ErrorKind::DisplayVersion => {
            println!("try {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    let command = args.command.clone().or_else(|| {
        if !args.query.is_empty() {
            let query = args.query.join(" ");
            if crate::commands::cd::looks_like_git_url(&query) {
                Some(cli::Command::Clone {
                    url: query.clone(),
                    name: None,
                })
            } else {
                Some(cli::Command::Cd { query: Some(query) })
            }
        } else {
            Some(cli::Command::Cd { query: None })
        }
    });

    let root = args.root_path()?;

    match command {
        Some(cli::Command::Init { path }) => {
            commands::init::init(path.clone())?;
        }
        Some(cli::Command::Cd { query }) => {
            commands::cd::cd(root, query.clone(), &args, cli::ExecutionMode::Direct)?;
        }
        Some(cli::Command::Exec { exec_command }) => match exec_command {
            cli::ExecCommand::Cd { query } => {
                commands::cd::cd(root, query.clone(), &args, cli::ExecutionMode::Script)?;
            }
            cli::ExecCommand::Clone { url, name } => {
                commands::clone::clone(
                    root,
                    url.clone(),
                    name.clone(),
                    cli::ExecutionMode::Script,
                )?;
            }
            cli::ExecCommand::Worktree { name } => {
                commands::worktree::worktree_dir(
                    root,
                    Some(name.clone()),
                    cli::ExecutionMode::Script,
                )?;
            }
        },
        Some(cli::Command::Clone { url, name }) => {
            commands::clone::clone(root, url.clone(), name.clone(), cli::ExecutionMode::Direct)?;
        }
        Some(cli::Command::Worktree { name }) | Some(cli::Command::Dot { name }) => {
            commands::worktree::worktree_dir(root, Some(name.clone()), cli::ExecutionMode::Direct)?;
        }
        None => {
            commands::cd::cd(root, None, &args, cli::ExecutionMode::Direct)?;
        }
    }

    Ok(())
}
