use anyhow::{Context, Result};
use log::info;
use std::{
    fs::File,
    io::{stdout, BufReader},
};
use structopt::StructOpt;
use clap_verbosity_flag::Verbosity;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for.
    pattern: String,

    /// The path to the file to read.
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(flatten)]
    verbose: Verbosity
}

fn main() -> Result<()> {
    env_logger::init();

    info!("reading file");

    let args = Cli::from_args();
    let path = &args.path;

    let f =
        File::open(path).with_context(|| format!("Could not read file `{}`", path.display()))?;

    let reader = BufReader::new(f);

    info!("finding matches");

    grrs::find_matches(reader, &args.pattern, &mut stdout().lock())?;

    Ok(())
}
