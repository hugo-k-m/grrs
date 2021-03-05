use anyhow::{Context, Result};
use log::info;
use std::io::{stdout, Write};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for.
    pattern: String,
    /// The path to the file to read.
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();

    info!("starting up");

    let args = Cli::from_args();
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Could not read file `{}`", path.display()))?;

    info!("finding matches");

    find_matches(content, &args.pattern, &mut stdout().lock())?;

    Ok(())
}

fn find_matches(content: String, pattern: &str, mut writer: impl Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}

#[test]
fn find_match() -> Result<()> {
    let mut result = Vec::new();
    find_matches(
        "lorem ipsum\ndolor sit amet".to_string(),
        "lorem",
        &mut result,
    )?;
    assert_eq!(result, b"lorem ipsum\n");

    Ok(())
}
