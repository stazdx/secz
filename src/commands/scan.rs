
use structopt::StructOpt;
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};

pub fn sonarRun() {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner());

    let mut child = Command::new("sonar-scanner")
            .arg("-Dsonar.rust.clippy.reportPaths=clippy-report.json")
            .arg("-Dsonar.login=admin")
            .arg("-Dsonar.password=admin123")
            // .output()
            .spawn()
            .expect("Failed to execute command");

    pb.set_message("Running sonar-scanner...");
    
    while let Ok(None) = child.try_wait() {
        pb.tick();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    pb.finish_with_message("sonarRun completed");

    // let child = String::from_utf8_lossy(&child.stdout);

    // println!("Output: {}", child);

}

#[derive(Debug, StructOpt)]
pub enum ResourceType {
    Sonar {},
    Trivy {
        a1: String,
        a2: u32,
    }
}

