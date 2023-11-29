use crate::commands::scan;

use clap::{Subcommand, Parser, ArgMatches};

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub comands: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[clap(flatten)]
    ScanCommands(scan::ScanCommands),
}