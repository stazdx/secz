use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "secz", about = "A security CLI tool.")]
pub struct Cli {
    /// The pattern to look for
    #[structopt(short = "p", long = "pattern")]
    pub pattern: String,

    /// The path to the file to read
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    pub path: PathBuf,
}

use std::io::{self, BufRead};
use std::fs::File;

pub fn run(cli: Cli) -> io::Result<()> {
    let file = File::open(&cli.path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to open file {}: {}", cli.path.display(), e))
    })?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(&cli.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}