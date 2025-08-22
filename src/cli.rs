//! Command-line utilities

use crate::report::BadgeStyle;
use clap::{ArgAction, ArgMatches, Command, arg, command};

pub const HELP_FILE: &str = r#"Coverage summary in JSON format"#;

pub const LONG_HELP_FILE: &str = r#"Coverage summary in JSON format.
With no FILE, or when FILE is -, read standard input.
To generate a coverage summary, use the following command:
cargo llvm-cov --json --summary-only"#;

pub const HELP_STYLE: &str = r#"Specify the badge style"#;

pub const LONG_HELP_STYLE: &str = r#"Specify the badge style.
Accepted styles are: flat, flat-square, plastic, for-the-badge or social.
Default style is flat."#;

/// Returns command-line arguments matches.
pub fn get_command() -> Command {
  command!().arg(arg!(<FILE>).help(HELP_FILE).long_help(LONG_HELP_FILE).required(false).index(1)).arg(
    arg!(--"style" <STYLE>)
      .short('s')
      .help(HELP_STYLE)
      .long_help(LONG_HELP_STYLE)
      .required(false)
      .action(ArgAction::Set)
      .display_order(1),
  )
}

/// Returns the name of the optional input file.
pub fn input_file(matches: &ArgMatches) -> Option<&String> {
  matches.get_one::<String>("FILE")
}

/// Returns the badge style.
pub fn badge_style(matches: &ArgMatches) -> BadgeStyle {
  if let Some(style) = matches.get_one::<String>("style") {
    style.as_str().into()
  } else {
    BadgeStyle::Default
  }
}
