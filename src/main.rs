use std::env;

use anyhow::*;

use minigrep::config_parser::parser;
use minigrep::search::parse_file;

fn main() -> Result<()> {
    let mut args = env::args();

    let config = parser::Config::build(&mut args)?;

    parse_file::run(config)?;
    Ok(())
}