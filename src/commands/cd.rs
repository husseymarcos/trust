use crate::context::RunContext;
use anyhow::Result;

const GIT_URL_PREFIXES: &[&str] = &["http://", "https://", "git@", "git://"];

pub fn cd(ctx: &RunContext<'_>, query: Option<String>) -> Result<()> {
    if let Some(q) = query.filter(|s| looks_like_git_url(s)) {
        return crate::commands::clone::clone(ctx, q, None);
    }
    anyhow::bail!("Interactive selector not yet implemented")
}

fn looks_like_git_url(s: &str) -> bool {
    GIT_URL_PREFIXES.iter().any(|p| s.starts_with(p)) || (s.contains("://") && s.contains(".git"))
}
