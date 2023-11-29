use clap::{Command, Args};
mod cli;

mod commands {
    pub mod scan;
}

fn main() {
    let cli = Command::new("secz");
    let command = cli::Cli::augment_args(cli).get_matches();

    let sub_matches = command.subcommand_matches("scan")
        .expect("Expected 'scan' subcommand");

    let tool = sub_matches.subcommand_name().unwrap_or("other");
    println!("Tool: {:?}", tool);
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
// }
