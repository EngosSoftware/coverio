//! Command-line utilities

use crate::report::{BadgeStyle, SeparatorStyle};
use clap::{ArgAction, ArgMatches, Command, arg, command};

pub const HELP_FILE: &str = r#"Coverage summary file in JSON format"#;

pub const LONG_HELP_FILE: &str = r#"Coverage summary file in JSON format.
With no FILE, or when FILE is -, read standard input.
To generate a coverage summary file, use the following command:
  cargo llvm-cov --json --summary-only"#;

pub const HELP_STYLE: &str = r#"Badge style"#;

pub const LONG_HELP_STYLE: &str = r#"Badge style:
'flat', 'flat-square', 'plastic', 'for-the-badge', 'social'"#;

pub const HELP_LABEL: &str = r#"Badge label"#;

pub const LONG_HELP_LABEL: &str = r#"Badge label."#;

pub const HELP_SEPARATOR: &str = r#"Value separator"#;

pub const LONG_HELP_SEPARATOR: &str = r#"Value separator:
'space', 'bar', 'spaced-bar'"#;

pub const HELP_NO_PERCENT_SIGN: &str = r#"Hide percent sign in values"#;

pub const LONG_HELP_NO_PERCENT_SIGN: &str = r#"Hide percent sign in values."#;

pub const HELP_COLLAPSE: &str = r#"Collapse to a single value if all are equal"#;

pub const LONG_HELP_COLLAPSE: &str = r#"Collapse to a single value if all are equal."#;

pub const HELP_MARKDOWN_TAG: &str = r#"Markdown tag to be updated"#;

pub const LONG_HELP_MARKDOWN_TAG: &str = r#"Markdown tag to be updated."#;

pub const HELP_MARKDOWN_FILE: &str = r#"Markdown file to be updated"#;

pub const LONG_HELP_MARKDOWN_FILE: &str = r#"Markdown file to be updated."#;

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
      arg!(--"separator" <SEPARATOR>)
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
    .arg(
      arg!(--"collapse")
        .short('c')
        .help(HELP_COLLAPSE)
        .long_help(LONG_HELP_COLLAPSE)
        .required(false)
        .action(ArgAction::SetTrue)
        .display_order(5),
    )
    .arg(
      arg!(--"tag" <TAG>)
        .short('t')
        .help(HELP_MARKDOWN_TAG)
        .long_help(LONG_HELP_MARKDOWN_TAG)
        .required(false)
        .action(ArgAction::Set)
        .display_order(6),
    )
    .arg(
      arg!(--"file" <FILE>)
        .short('f')
        .help(HELP_MARKDOWN_FILE)
        .long_help(LONG_HELP_MARKDOWN_FILE)
        .required(false)
        .action(ArgAction::Set)
        .requires("tag")
        .display_order(7),
    )
}

/// Returns the name of the optional input file.
pub fn input_file(matches: &ArgMatches) -> Option<String> {
  matches.get_one::<String>("FILE").cloned()
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

/// Returns flag indicating if percent sign should be hidden.
pub fn no_percent_sign(matches: &ArgMatches) -> bool {
  matches.get_one::<bool>("no-percent-sign").unwrap().to_owned()
}

pub fn collapse(matches: &ArgMatches) -> bool {
  matches.get_one::<bool>("collapse").unwrap().to_owned()
}

/// Returns the badge tag in Markdown file.
pub fn markdown_tag(matches: &ArgMatches) -> Option<String> {
  matches.get_one::<String>("tag").cloned()
}

/// Returns the badge tag in Markdown file.
pub fn markdown_file(matches: &ArgMatches) -> Option<String> {
  matches.get_one::<String>("file").cloned()
}
