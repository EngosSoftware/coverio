//! Command-line utilities

use crate::report::{BadgeStyle, SeparatorStyle};
use clap::{ArgAction, ArgMatches, Command, arg, command};

pub const HELP_FILE: &str = r#"Coverage summary in JSON format"#;

pub const LONG_HELP_FILE: &str = r#"Coverage summary in JSON format.
With no FILE, or when FILE is -, read standard input.
To generate a coverage summary, use the following command:
cargo llvm-cov --json --summary-only"#;

pub const HELP_STYLE: &str = r#"Specify the badge style"#;

pub const LONG_HELP_STYLE: &str = r#"Specify the badge style.
Accepted styles are: flat, flat-square, plastic, for-the-badge or social.
Default style is 'flat'."#;

pub const HELP_LABEL: &str = r#"Specify the badge label"#;

pub const LONG_HELP_LABEL: &str = r#"Specify the badge label."#;

pub const HELP_SEPARATOR: &str = r#"Specify coverage value separator"#;

pub const LONG_HELP_SEPARATOR: &str = r#"Specify coverage value separator.
Accepted separators are: space, bar, spaced-bar."#;

/// Returns command-line arguments matches.
pub fn get_command() -> Command {
  command!()
    .arg(arg!(<FILE>).help(HELP_FILE).long_help(LONG_HELP_FILE).required(false).index(1))
    .arg(
      arg!(--"style" <STYLE>)
        .short('s')
        .help(HELP_STYLE)
        .long_help(LONG_HELP_STYLE)
        .required(false)
        .action(ArgAction::Set)
        .display_order(1),
    )
    .arg(
      arg!(--"label" <LABEL>)
        .short('l')
        .help(HELP_LABEL)
        .long_help(LONG_HELP_LABEL)
        .required(false)
        .action(ArgAction::Set)
        .default_value("cov")
        .display_order(2),
    )
    .arg(
      arg!(--"separator" <LABEL>)
        .short('r')
        .help(HELP_SEPARATOR)
        .long_help(LONG_HELP_SEPARATOR)
        .required(false)
        .action(ArgAction::Set)
        .default_value("spaced-bar")
        .display_order(3),
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

/// Returns the badge label.
pub fn badge_label(matches: &ArgMatches) -> String {
  matches.get_one::<String>("label").unwrap().to_string()
}

/// Returns the coverage value separator.
pub fn separator_style(matches: &ArgMatches) -> SeparatorStyle {
  matches.get_one::<String>("separator").unwrap().as_str().into()
}
