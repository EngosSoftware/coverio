//! Command-line utilities

use clap::{ArgMatches, Command, arg, command};

pub const HELP_FILE: &str = r#"Coverage summary in JSON format"#;

pub const LONG_HELP_FILE: &str = r#"Coverage summary in JSON format.
With no FILE, or when FILE is -, read standard input.
To generate a coverage summary, use the following command:
cargo llvm-cov --json --summary-only"#;

/// Returns command-line arguments matches.
pub fn get_command() -> Command {
  command!().arg(arg!(<FILE>).help(HELP_FILE).long_help(LONG_HELP_FILE).required(false).index(1))
}

/// Returns the name of the optional input file.
pub fn input_file(matches: &ArgMatches) -> Option<&String> {
  matches.get_one::<String>("FILE")
}
