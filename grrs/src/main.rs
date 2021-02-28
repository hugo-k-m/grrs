use anyhow::{Context, Result};
use std::io::{self, Write};
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Could not read file `{:?}`", path))?;

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?;
        }
    }

    Ok(())
}
