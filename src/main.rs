use std::fs::File;
use std::io::{BufRead,self, BufReader, Result};
use std::path::PathBuf;
use std::time::Duration;
use indicatif::ProgressBar;
use std::thread;
use log::{info, warn};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn answer() -> i32 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42)
}


fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        let line = line;

        if line.contains(pattern) {
            info!("{}", line);
        }
    }

    Ok(())
}

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    let pb = ProgressBar::new(100);
    for _i in 0..100 {
        // let _ = run();
        // pb.println(format!("[+] finished #{}", i));
        thread::sleep(Duration::from_millis(10));
        pb.inc(1);
    }

    pb.finish_with_message("done");

    let _ = run();
}

fn run() -> io::Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.path)?;
    let reader = io::BufReader::new(file);
    let mut line = String::new();
    let _ = reader.read_line(line);

    find_matches(&reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}