//! Command-line utilities

use crate::report::BadgeProperties;
use antex::{StyledText, Text};
use clars::{Clar, ClarArgument, ClarOption};

const NAME: &str = env!("CARGO_PKG_NAME");

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_FILE: &str = "Coverage summary file in JSON format or '-'";

const LONG_HELP_FILE: &str = r"Coverage summary file in JSON format.
With no FILE, or when FILE is -, read standard input.
To generate a coverage summary file, use the following command:
  cargo llvm-cov --json --summary-only";

const HELP_LABEL: &str = r"Badge label";

const HELP_NO_PERCENT_SIGN: &str = "Hide percent sign in values";

pub const HELP_MARKDOWN_TAG: &str = "Markdown tag to be updated";

pub const HELP_MARKDOWN_FILE: &str = "Markdown file to be updated";

const BADGE_STYLE_DEFAULT: &str = "flat";

const BADGE_LABEL_DEFAULT: &str = "coverage";

const SEPARATOR_DEFAULT: &str = "spaced-bar";

/// Command-line action.
pub enum CliAction {
    Run(
        /// Input file name.
        Option<String>,
        /// Badge properties.
        BadgeProperties,
        /// Markdown tag.
        Option<String>,
        /// Markdown file.
        Option<String>,
    ),
    ShowHelp(
        /// Help text.
        Text,
    ),
    ShowVersion(
        /// Version description.
        Text,
    ),
    Error(
        /// Error details.
        Text,
    ),
}

fn get_cli(app: String) -> Clar {
    Clar::new(app).description(DESCRIPTION).options_arguments(
        [
            ClarOption::short("badge-style", 's')
                .long_label("style")
                .takes_value("STYLE")
                .default_value(BADGE_STYLE_DEFAULT)
                .possible_values(["flat", "flat-square", "plastic", "for-the-badge", "social"])
                .help("Badge style"),
            ClarOption::short("badge-label", 'l')
                .long_label("label")
                .takes_value("LABEL")
                .default_value(BADGE_LABEL_DEFAULT)
                .help(HELP_LABEL),
            ClarOption::short("separator", 'r')
                .long_label("separator")
                .takes_value("SEPARATOR")
                .default_value(SEPARATOR_DEFAULT)
                .possible_values(["space", "bar", "spaced-bar"])
                .help(HELP_LABEL),
            ClarOption::short("collapse", 'C')
                .long_label("collapse")
                .takes_value("COLLAPSE")
                .default_value("equal")
                .possible_values(["never", "equal", "regions", "functions", "lines"])
                .help("Collapse to a single value"),
            ClarOption::short("no-percent-sign", 'n')
                .long_label("no-percent-sign")
                .help(HELP_NO_PERCENT_SIGN),
            ClarOption::short("markdown-tag", 't')
                .long_label("tag")
                .takes_value("TAG")
                .help(HELP_MARKDOWN_TAG),
            ClarOption::short("markdown-file", 'f')
                .long_label("file")
                .takes_value("FILE")
                .help(HELP_MARKDOWN_FILE),
            ClarOption::short("version", 'V')
                .long_label("version")
                .standalone()
                .help_short("Print version"),
            ClarOption::short("help", 'h')
                .long_label("help")
                .standalone()
                .help("Print help"),
        ],
        [ClarArgument::new("input-file")
            .caption("FILE")
            .help(HELP_FILE)
            .help_long(LONG_HELP_FILE)],
    )
}

pub fn get_action(app: String, args: Vec<String>) -> CliAction {
    match get_cli(app).resolve(args) {
        Ok(matches) => {
            if matches.is_short("help") {
                return CliAction::ShowHelp(matches.get_help());
            }
            if matches.is_long("help") {
                return CliAction::ShowHelp(matches.get_help_long());
            }
            if matches.is_present("version") {
                let version = Text::default()
                    .bright_cyan()
                    .bold()
                    .s(NAME)
                    .r(" v")
                    .s(VERSION);
                return CliAction::ShowVersion(version);
            }
            let input_file = matches.get_first_value("input-file");
            let badge_style = matches
                .get_first_value("badge-style")
                .unwrap()
                .as_str()
                .into();
            let badge_label = matches.get_first_value("badge-label").unwrap();
            let separator = matches
                .get_first_value("separator")
                .unwrap()
                .as_str()
                .into();
            let collapse = matches.get_first_value("collapse").unwrap().as_str().into();
            let no_percent_sign = matches.is_present("no-percent-sign");
            let badge_properties = BadgeProperties::new(
                badge_style,
                badge_label,
                separator,
                no_percent_sign,
                collapse,
            );
            let markdown_tag = matches.get_first_value("markdown-tag");
            let markdown_file = matches.get_first_value("markdown-file");
            CliAction::Run(input_file, badge_properties, markdown_tag, markdown_file)
        }
        Err(diagnostic) => CliAction::Error(diagnostic.text().clone()),
    }
}
