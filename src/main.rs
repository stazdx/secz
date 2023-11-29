// use structopt::StructOpt;
use clap::{Command, Args, ArgMatches};
mod cli;

mod commands {
    pub mod scan;
}

fn main() {
    let mut cli = Command::new("secz");
    cli = cli::Cli::augment_args(cli);
    // println!("{:?}", cli);
cli.get_matches().get_one(id)
    let name = cli.get_matches();
    print!("{:?}", name);

    match name.subcommand() {
        Some("scan") => {
            println!("Scan");
        },
        Some(("scan", name)) => {
            if name.get_one("sonar") {
                println!("Scan sonar");
                secz::commands::scan::sonarRun();
            } else if name.get_one("trivy") {
                println!("Scan trivy");
                // secz::commands::scan::Trivy();
            } else {
                eprintln!("Unknown tool");
            }
            // Some(("sonar", name.get_one("sonar"))) => {
            //     println!("Scan sonar");
            //     secz::commands::scan::sonarRun();
            // },
            // Some(("trivy", name.get_one("trivy"))) => {
            //     println!("Scan trivy");
            //     // secz::commands::scan::Trivy();
            // },
        },
        _ => {
            eprintln!("Unknown command");
        }
    }

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
