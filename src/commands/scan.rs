
// use structopt::StructOpt;
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    trivy: TrivyConfig,
    sonar: SonarConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TrivyConfig {
    enabled: bool,
    args: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SonarConfig {
    enabled: bool,
    args: Vec<String>,
}

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

pub fn read_yaml() -> Result<(), serde_yaml::Error> {
    let mut file = File::open("secz.yaml").expect("Unable to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    let config: Config = serde_yaml::from_str(&contents["config:".len()..])?;
    println!("{:?}", &config);

    Ok(())
}

pub fn sonar_run() {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner());

    let args = [
        "-Dsonar.rust.clippy.reportPaths=clippy-report.json",
        "-Dsonar.login=admin",
        "-Dsonar.password=admin123"
        ];

    let mut child = Command::new("sonar-scanner")
            .args(&args)
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