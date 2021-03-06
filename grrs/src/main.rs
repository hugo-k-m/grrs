use anyhow::{Context, Result};
use log::info;
use std::io::stdout;
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

    grrs::find_matches(content, &args.pattern, &mut stdout().lock())?;

    Ok(())
}

#[test]
fn find_match() -> Result<()> {
    let mut result = Vec::new();
    grrs::find_matches(
        "lorem ipsum\ndolor sit amet".to_string(),
        "lorem",
        &mut result,
    )?;
    assert_eq!(result, b"lorem ipsum\n");

    Ok(())
}
