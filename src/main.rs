use secz::cli;
use structopt::StructOpt;
// use secz::commands::scan::ResourceType; 

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

    if let Err(err) = cli::run(cli) {
        eprintln!("Error: {}", err);
    }

    // cli::run(cli);
}
