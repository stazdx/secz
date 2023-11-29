
// use structopt::StructOpt;
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum ScanCommands {
    #[clap(subcommand, about = "Scan with a tool", name = "scan")]
    ToolType(ToolType),
    #[clap(subcommand, about = "Scan with a tool2", name = "test")]
    Test(ToolType),
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolType {
    Sonar(SonarTool),
    Trivy(TrivyTool),
}

#[derive(Debug, Clone, Parser)]
pub struct SonarTool {
    #[arg(id = "sonar",long, default_value = "sonar-scanner", ignore_case = true)]
    pub sonar: String,
}

#[derive(Debug, Clone, Parser)]
pub struct TrivyTool {
    #[arg(long, default_value = "trivy", ignore_case = true)]
    pub trivy: String,
}

pub fn sonar_run() {
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