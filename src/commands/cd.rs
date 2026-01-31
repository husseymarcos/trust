use crate::context::RunContext;
use anyhow::Result;

const GIT_URL_PREFIXES: &[&str] = &["http://", "https://", "git@", "git://"];

pub fn cd(ctx: &RunContext, query: Option<String>) -> Result<()> {
    if let Some(ref q) = query.as_ref().filter(|s| looks_like_git_url(s)) {
        return crate::commands::clone::clone(ctx, q.to_string(), None);
    }
    crate::tui::run(ctx, query)
}

fn looks_like_git_url(s: &str) -> bool {
    GIT_URL_PREFIXES.iter().any(|p| s.starts_with(p)) || (s.contains("://") && s.contains(".git"))
}
