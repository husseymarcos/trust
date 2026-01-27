use anyhow::Result;
use std::path::PathBuf;

pub fn init(path: Option<PathBuf>) -> Result<()> {
    let root = path
        .or_else(|| std::env::var("TRY_PATH").ok().map(PathBuf::from))
        .ok_or_else(|| anyhow::anyhow!("PATH required (either as argument or TRY_PATH env var)"))?;

    let root_str = root.to_string_lossy();

    let shell = detect_shell()?;
    let function = match shell.as_str() {
        "fish" => fish_function(&root_str),
        "zsh" => zsh_function(&root_str),
        _ => bash_function(&root_str),
    };

    println!("{function}");
    Ok(())
}

fn detect_shell() -> Result<String> {
    let shell = std::env::var("SHELL")?;
    let shell_name = std::path::Path::new(&shell)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("bash")
        .to_string();

    Ok(shell_name)
}

fn bash_function(root: &str) -> String {
    format!(
        r#"try() {{
    local output
    export TRY_PATH="{}"
    output=$("{}" "$@")
    if [ -n "$output" ]; then
        eval "$output"
    fi
}}"#,
        root,
        std::env::current_exe()
            .unwrap_or_else(|_| "try".into())
            .to_string_lossy()
    )
}

fn zsh_function(root: &str) -> String {
    format!(
        r#"try() {{
    local output
    export TRY_PATH="{}"
    output=$("{}" "$@")
    if [ -n "$output" ]; then
        eval "$output"
    fi
}}"#,
        root,
        std::env::current_exe()
            .unwrap_or_else(|_| "try".into())
            .to_string_lossy()
    )
}

fn fish_function(root: &str) -> String {
    format!(
        r#"function try
    set -x TRY_PATH "{}"
    set output ({} $argv)
    if [ -n "$output" ]
        eval $output
    end
end"#,
        root,
        std::env::current_exe()
            .unwrap_or_else(|_| "try".into())
            .to_string_lossy()
    )
}
