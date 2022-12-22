use anyhow::Result;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ConfigParserError {
    #[error("Invalid arguments provided")]
    InvalidArguments,
    #[error("File specified could not be found")]
    FileNotFound,
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: PathBuf,
    pub is_dir: bool,
}

impl Config {
    fn new(query: String, file_path: PathBuf, is_dir: bool) -> Config {
        Config {
            query,
            file_path,
            is_dir,
        }
    }
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config> {
        let file_path: PathBuf;
        let mut is_dir = false;
        // Check out clap
        // Skip the executable name
        if args.next().is_none() {
            Err(ConfigParserError::InvalidArguments)?;
        }
        // Get the query
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(ConfigParserError::InvalidArguments)?,
        };
        // Get the filepath if provided, otherwise specify the file path is the
        // current directory
        match args.next() {
            Some(arg) => {
                // TODO: How can I mock this?
                file_path = PathBuf::from(arg);
                if !file_path.exists() {
                    eprintln!("File path is not valid.");
                    return Err(ConfigParserError::FileNotFound)?;
                }
                if file_path.is_dir() {
                    is_dir = true;
                }
                else if !file_path.is_file() {
                    eprintln!("File path is neither directory nor file.");
                    return Err(ConfigParserError::FileNotFound)?;
                }
            }
            None => {
                file_path = std::env::current_dir().unwrap();
                is_dir = true;
            }
        }
        Ok(Config::new(query, file_path, is_dir))
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn test_build_no_args() {
        let args = vec!["target\\debug\\minigrep.exe"];
        let mut args = args.iter().map(|item| item.to_string());
        let error = Config::build(&mut args).unwrap_err();
        // Downcast used to convert to concrete type
        // Can use Error::Is to check if it is a concrete type
        assert_eq!(error.downcast::<ConfigParserError>().unwrap(), ConfigParserError::InvalidArguments);
    }

    #[test]
    fn test_build_provide_file() {
        let args = vec!["target\\debug\\minigrep.exe", "another"];
        let mut args = args.iter().map(|item| item.to_string());
        let config = Config::build(&mut args).expect("Expect valid config");
        assert!(config.query == "another");
        assert!(config.file_path == std::env::current_dir().unwrap());
        assert!(config.is_dir == true);
    }
}
