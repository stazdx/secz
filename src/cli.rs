use std::path::PathBuf;
use structopt::StructOpt;
use crate::commands::scan::ResourceType;

#[derive(Debug, StructOpt)]
#[structopt(name = "secz", about = "A security CLI tool.")]
pub struct Cli {

    #[structopt(subcommand)]
    pub cmd: Command,

    #[structopt(short = "p", long = "pattern")]
    pub pattern: String,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    pub path: PathBuf,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Scan {
        #[structopt(subcommand)]
        resource: ResourceType,
        // #[structopt(short = "a", long = "all")]
        // all: String,
    },
    Delete {
        all: String,
    },

}

// #[derive(Debug, StructOpt)]
// pub enum ResourceType {
//     Sonar {
//         applications: u32,
//     },
//     Trivy {
//         a1: String,
//         a2: u32,
//     }
// }

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