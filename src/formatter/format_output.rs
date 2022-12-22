use std::path::Path;

use crate::search::parse_file::LineResult;

use colored::Colorize;
use thiserror::Error;
use anyhow::Result;

#[derive(Error, Debug)]
pub enum FormatterError {
    #[error("Path specified was not a proper file")]
    InvalidFile
}

/// Formats results when the user queried a single file
///
/// Displays every line match for the file
pub fn format_results(results: &[LineResult]) {
    for item in results {
        println!("{}: {}", format!("{}", item.line_number).green(), item.line);
    }
}

/// Formats results for a given file when the user queried multiple files
///
/// Displays the file name before printing each line match found
pub fn format_results_dir(file_path: &Path, results: &[LineResult]) -> Result<()>{
    let local_root = std::env::current_dir().unwrap();
    let relative_path = match file_path.strip_prefix(local_root) {
        Ok(relative_path) => relative_path,
        Err(_) => return Err(FormatterError::InvalidFile)?
    };
    print_file_name(relative_path.to_str().unwrap_or(""));
    for item in results {
        println!("{}: {}", format!("{}", item.line_number).green(), item.line);
    }
    Ok(())
}

fn print_file_name(file_name: &str) {
    println!("{}", file_name.blue());
}

#[cfg(test)]
mod tests {
    // use super::*;
}
