
use structopt::StructOpt;
use std::process::Command;

pub fn sonarRun() {
    let output = Command::new("sonar-scanner")
            .arg("-Dsonar.rust.clippy.reportPaths=clippy-report.json")
            .arg("-Dsonar.login=admin")
            .arg("-Dsonar.password=admin123")
            .output()
            .expect("Failed to execute command");

    let output = String::from_utf8_lossy(&output.stdout);

    println!("Output: {}", output);

}


#[derive(Debug, StructOpt)]
pub enum ResourceType {
    Sonar {},
    Trivy {
        a1: String,
        a2: u32,
    }
}

