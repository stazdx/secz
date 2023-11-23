use secz::cli;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_something() {
        // Arrange

        let cli = cli::Cli {
            pattern: "test".to_string(),
            path: PathBuf::from("non_existing_file.txt"),
        };
        // Assert
        assert!(cli::run(cli).is_err());
    }

    // Add more tests here...
}