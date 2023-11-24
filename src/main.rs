use secz::cli;
use structopt::StructOpt;
use secz::commands::scan;

mod commands {
    pub mod scan;
}

fn main() {
    println!("Hello, world!");
    // let cli = cli::Cli {
    //     pattern: "test".to_string(),
    //     path: std::path::PathBuf::from("file.txt"),
    // };

    let cli = cli::Cli::from_args();

    println!("{:?}", cli);

    match cli.cmd {
        cli::Command::Scan { resource } => {
            match resource {
                scan::ResourceType::Sonar {} => {
                    println!("Sonar");
                    scan::sonarRun();
                },
                scan::ResourceType::Trivy { a1, a2 } => {
                    println!("Trivy");
                },
            }
        },
        cli::Command::Delete { all } => {
            println!("Delete");
        },
    }

    // if let Err(err) = cli::run(cli) {
    //     eprintln!("Error: {}", err);
    // } else {
    //     println!("Success!");
    // }

    // cli::run(cli);
}
