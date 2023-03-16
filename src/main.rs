#![forbid(unsafe_code)]

mod count;

use std::io::{self, BufWriter};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to count in.
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let counters = count::find_unsafe_recursively(&args.path)?;

    let stdout = BufWriter::new(io::stdout().lock());
    serde_json::to_writer_pretty(stdout, &counters)?;

    Ok(())
}
