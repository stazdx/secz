
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum ResourceType {
    Sonar {
        applications: u32,
    },
    Trivy {
        a1: String,
        a2: u32,
    }
}