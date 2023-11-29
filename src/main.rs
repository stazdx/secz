// use structopt::StructOpt;
use clap::{Command, Args};
mod cli;

mod commands {
    pub mod scan;
}

fn main() {
    let cli = Command::new("secz");

    let command = cli::Cli::augment_args(cli).get_matches();

    match command.subcommand_matches("scan") {
        Some(sub_matches) => {
            let tool = if sub_matches.subcommand_matches("sonar").is_some() {
                "sonar"
            } else {
                "other"
            };
            println!("Tool: {:?}", tool);
        }
        None => {
            eprintln!("Unknown command");
        },
    }
}
    // match command.subcommand() {
    //     Some(("scan", sub_matches)) => {
    //         println!("{}", command.);
    //         let tool = if command.contains_id("sonar") {
    //             "sonar"
    //         } else {
    //             "other"
    //         };
    //         println!("Tool: {}", tool);
    //     },
    //     Some(("test", _sub_matches)) => {
    //         println!("Test");
            // if name.get_one("sonar") {
            //     println!("Scan sonar");
            //     secz::commands::scan::sonarRun();
            // } else if name.get_one("trivy") {
            //     println!("Scan trivy");
            //     // secz::commands::scan::Trivy();
            // } else {
            //     eprintln!("Unknown tool");
            // }
            // Some(("sonar", name.get_one("sonar"))) => {
            //     println!("Scan sonar");
            //     secz::commands::scan::sonarRun();
            // },
            // Some(("trivy", name.get_one("trivy"))) => {
            //     println!("Scan trivy");
            //     // secz::commands::scan::Trivy();
            // },
        // },
        // _ => {
        //     eprintln!("Unknown command");
        // }
    // }

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
// }
