use anyhow::Result;
use std::path::PathBuf;

fn exe() -> String {
    std::env::current_exe()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "try".to_string())
}

pub fn init(path: Option<PathBuf>) -> Result<()> {
    let root = path
        .or_else(|| std::env::var("TRY_PATH").ok().map(PathBuf::from))
        .ok_or_else(|| anyhow::anyhow!("PATH required (either as argument or TRY_PATH env var)"))?;
    let root_str = root.to_string_lossy();
    let shell = std::env::var("SHELL")
        .ok()
        .and_then(|s| {
            std::path::Path::new(&s)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "bash".to_string());
    let function = match shell.as_str() {
        "fish" => fish_function(&root_str),
        _ => sh_function(&root_str),
    };
    println!("{function}");
    Ok(())
}

fn sh_function(root: &str) -> String {
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
        exe()
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
        exe()
    )
}
