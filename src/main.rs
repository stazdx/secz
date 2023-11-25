// use structopt::StructOpt;
use clap::{Command, Args};
mod cli;

mod commands {
    pub mod scan;
}

fn main() {
    println!("Hello, world!");
    // let cli = cli::Cli {
    //     pattern: "test".to_string(),
    //     path: std::path::PathBuf::from("file.txt"),
    // };

    // let cli = cli::Cli::from_args();

    let mut cli = Command::new("secz");
    cli = cli::Cli::augment_args(cli);
    println!("{:?}", cli);

    let name = cli.get_matches();
    print!("{:?}", name);
    // match cli.get_matches().subcommand() {
    //     secz::commands::scan::ScanCommands::ToolType(tool) => {
    //         if let Some(("tool", matches)) = cli.get_matches().subcommand() {
    //             match matches.value_of("tool") {
    //                 Some("sonar") => {
    //                     println!("Sonar");
    //                     // scan::sonarRun();
    //                 },
    //                 Some("trivy") => {
    //                     println!("Trivy");
    //                 },
    //                 _ => {
    //                     eprintln!("Unknown tool");
    //                 },
    //             }
    //         }
    //     }
    // }
}
