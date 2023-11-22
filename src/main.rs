mod cli;

fn main() {
    println!("Hello, world!");
    let cli = cli::Cli {
        pattern: "foo".to_string(),
        path: std::path::PathBuf::from("bar.txt"),
    };

    println!("{:?}", cli)
}
