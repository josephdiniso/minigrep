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
        format_line(item);
    }
}

/// Formats results for a given file when the user queried multiple files
///
/// Displays the file name before printing each line match found
pub fn format_results_dir(file_path: &Path, results: &[LineResult]) -> Result<()>{
    let local_root = std::env::current_dir().unwrap();
    let relative_path = match file_path.strip_prefix(local_root) {
        Ok(relative_path) => relative_path,
        Err(_) => file_path
    };
    print_file_name(relative_path.to_str().unwrap_or(""));
    for item in results {
        format_line(item);
    }
    Ok(())
}

fn format_line(line_result: &LineResult) {
    // https://developer-book.com/post/definitive-guide-for-colored-text-in-terminal/#:~:text=If%20a%20terminal%20application%20wants%20to%20print%20colored,to%20reset%20the%20text%20color%20settings%20for%20terminal.
    let mut is_matched = vec![false; line_result.line.len()];
    for re_match in line_result.matches.iter() {
        for item in is_matched.iter_mut().take(re_match.1).skip(re_match.0) {
            *item = true;
        }
    }
    let mut matching = false;
    let mut new_str = format!("\x1b[32m{}\x1b[0m: ", line_result.line_number);
    for (index, character) in line_result.line.chars().enumerate() {
        if is_matched[index] && !matching {
            matching = true;
            new_str += "\x1b[31m"
        }
        else if !is_matched[index] && matching {
            matching = false;
            new_str += "\x1b[0m"
        }
        new_str += &character.to_string();
    }
    println!("{}", new_str);
}

fn print_file_name(file_name: &str) {
    println!("{}", file_name.blue());
}

#[cfg(test)]
mod tests {
    // use super::*;
}
