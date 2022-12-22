use std::fs;
use std::result::Result::Ok;

use anyhow::*;
use regex::RegexBuilder;
use walkdir::WalkDir;

use crate::config_parser::parser::ConfigParserError;
use crate::formatter::format_output;

#[derive(Debug)]
pub struct LineResult<'a> {
    pub line_number: i64,
    pub line: &'a str,
    pub matches: Vec<(usize, usize)>,
}

pub struct FileResult<'a> {
    pub file_name: String,
    pub line_results: Vec<LineResult<'a>>,
}

impl<'a> LineResult<'a> {
    pub fn new(line_number: i64, line: &'a str, matches: Vec<(usize, usize)>) -> LineResult {
        LineResult {
            line_number,
            line,
            matches,
        }
    }
}

fn find_matches_in_dir_child_files(config: &crate::config_parser::parser::Config) -> Result<()> {
    // TODO: Replace this with a self made one
    let mut had_match = false;
    let Ok(re) = build_query(&config.query, Some(true)) else {
        return Err(ConfigParserError::InvalidArguments)?;
    };
    WalkDir::new(&config.file_path)
        .into_iter()
        .for_each(|entry| {
            let Ok(entry_path) = entry else {
                eprintln!("Could not get path");
                return;
            };
            let entry_path = entry_path.path();
            if !entry_path.is_file()
            {
                return;
            }
            let Ok(contents) = fs::read_to_string(entry_path) else {
                return;
            };
            let Ok(results) = search(&re, &contents) else {
                return;
            };
            if results.is_empty() {
                return;
            }
            if had_match {
                println!();
            }
            had_match = true;
            if format_output::format_results_dir(entry_path, &results).is_ok() {}
        });
    Ok(())
}

pub fn run(config: crate::config_parser::parser::Config) -> Result<()> {
    if config.is_dir {
        if find_matches_in_dir_child_files(&config).is_err() {
            eprintln!("Find matches in child files failed");
        }
    } else {
        let Ok(re) = build_query(&config.query, Some(true)) else {
            eprintln!("Query invalid");
            return Err(ConfigParserError::InvalidArguments)?;
        };
        let contents = match fs::read_to_string(config.file_path) {
            Ok(contents) => contents,
            Err(_) => {
                eprintln!("Invalid arguments: could not read file");
                return Err(crate::config_parser::parser::ConfigParserError::InvalidArguments)?;
            }
        };
        let results = match search(&re, &contents) {
            Ok(result) => result,
            Err(_) => {
                return Err(crate::config_parser::parser::ConfigParserError::InvalidArguments)?
            }
        };
        format_output::format_results(&results);
    }

    Ok(())
}

/// Build regex query
fn build_query(query: &str, case_insensitive: Option<bool>) -> Result<regex::Regex> {
    let case_insensitive = case_insensitive.unwrap_or(true);
    let regex = RegexBuilder::new(query)
        .case_insensitive(case_insensitive)
        .build()?;
    Ok(regex)
}

fn search<'a>(re: &regex::Regex, contents: &'a str) -> Result<Vec<LineResult<'a>>> {
    let mut results = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        if re.is_match(line) {
            let matches: Vec<(usize, usize)> = re
                .find_iter(line)
                .map(|re_match| (re_match.start(), re_match.end()))
                .collect();
            results.push(LineResult::new((line_number + 1) as i64, line, matches));
        }
    }

    Ok(results)
}

// TODO: Fix these tests
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let re = build_query(&query, Some(true)).unwrap();
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(
//             vec![LineResult {
//                 line_number: 2,
//                 line: "safe, fast, productive."
//             }],
//             search(&re, contents).unwrap()
//         );
//     }

//     #[test]
//     fn two_results() {
//         let query = "fast";
//         let re = build_query(&query, Some(true)).unwrap();
//         let contents = "\
// fast Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(
//             vec![
//                 LineResult {
//                     line_number: 1,
//                     line: "fast Rust:"
//                 },
//                 LineResult {
//                     line_number: 2,
//                     line: "safe, fast, productive."
//                 }
//             ],
//             search(&re, contents).unwrap()
//         );
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "DuCt";
//         let re = build_query(&query, Some(true)).unwrap();
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(
//             vec![LineResult {
//                 line_number: 2,
//                 line: "safe, fast, productive."
//             }],
//             search(&re, contents).unwrap()
//         );
//     }
// }
