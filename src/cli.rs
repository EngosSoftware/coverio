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
Valid styles: 'flat', 'flat-square', 'plastic', 'for-the-badge', 'social'"#;

pub const HELP_LABEL: &str = r#"Specify the badge label"#;

pub const LONG_HELP_LABEL: &str = r#"Specify the badge label."#;

pub const HELP_SEPARATOR: &str = r#"Specify coverage value separator"#;

pub const LONG_HELP_SEPARATOR: &str = r#"Specify coverage value separator.
Valid separators: 'space', 'bar', 'spaced-bar'"#;

pub const HELP_NO_PERCENT_SIGN: &str = r#"Hide percent sign for coverage values"#;

pub const LONG_HELP_NO_PERCENT_SIGN: &str = r#"Hide percent sign for coverage values"#;

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
        .default_value("flat")
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
    .arg(
      arg!(--"no-percent-sign")
        .short('n')
        .help(HELP_NO_PERCENT_SIGN)
        .long_help(LONG_HELP_NO_PERCENT_SIGN)
        .required(false)
        .action(ArgAction::SetTrue)
        .display_order(4),
    )
}

/// Returns the name of the optional input file.
pub fn input_file(matches: &ArgMatches) -> Option<&String> {
  matches.get_one::<String>("FILE")
}

/// Returns the badge style.
pub fn badge_style(matches: &ArgMatches) -> BadgeStyle {
  matches.get_one::<String>("style").unwrap().as_str().into()
}

/// Returns the badge label.
pub fn badge_label(matches: &ArgMatches) -> String {
  matches.get_one::<String>("label").unwrap().to_string()
}

/// Returns the coverage value separator.
pub fn separator_style(matches: &ArgMatches) -> SeparatorStyle {
  matches.get_one::<String>("separator").unwrap().as_str().into()
}

pub fn no_percent_sign(matches: &ArgMatches) -> bool {
  matches.get_one::<bool>("no-percent-sign").unwrap().to_owned()
}
