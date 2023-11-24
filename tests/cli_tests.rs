use secz::cli;
use secz::commands::scan;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use secz::commands;

    use super::*;

    #[test]
    fn test_something() {
        // Arrange

        let cli = cli::Cli {
            cmd: cli::Command::Scan {
                resource: scan::ResourceType::Sonar {
                    applications: 1,
                },
                // all: "all".to_string(),
            },
            pattern: "test".to_string(),
            path: PathBuf::from("non_existing_file.txt"),
        };
        // Assert
        assert!(cli::run(cli).is_err());
    }

    // Add more tests here...
}