mod cli;

fn main() {
    println!("Hello, world!");
    let cli = cli::Cli {
        pattern: "test".to_string(),
        path: std::path::PathBuf::from("file.txt"),
    };

    println!("{:?}", cli);

    if let Err(err) = cli::run(cli) {
        eprintln!("Error: {}", err);
    }

    // cli::run(cli);
}
