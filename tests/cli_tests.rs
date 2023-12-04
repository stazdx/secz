use secz::cli;
use secz::commands::scan;
use clap::{Command, Args};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        // Simulate command line arguments
        let args = vec!["secz", "scan", "sonar"];

        // Set up the command line arguments
        let mut cli = Command::new("secz");
        cli.set_args(args);

        // Call the main function
        main();

        // Assert that the correct tool is printed
        assert_eq!(std::io::stdout().to_string(), "Tool: Some(\"sonar\")\n");
    }
}