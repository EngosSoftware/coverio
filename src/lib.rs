#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../docs/lib.rs.md")]

mod cli;
mod errors;
mod report;

pub use errors::{CoverioError, Result};

use crate::cli::{CliAction, get_action};
use antex::ColorMode;
use clars::{get_app_and_args, get_more_info_hint};
use report::{BadgeProperties, CoverageReport};
use std::io::Read;
use std::process::ExitCode;
use std::{fs, io};

fn get_content_from_file(file_name: String) -> Result<String> {
    fs::read_to_string(file_name).map_err(|e| CoverioError::new(e.to_string()))
}

fn get_content_from_stdin() -> Result<String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|e| CoverioError::new(e.to_string()))?;
    Ok(content)
}

fn process_content(content: &str, badge_properties: &BadgeProperties) -> Result<String> {
    let json: serde_json::Value =
        serde_json::from_str(content).map_err(|e| CoverioError::new(e.to_string()))?;
    let mut report = CoverageReport::new();
    report.analyze(&json)?;
    let badge_link = report.badge(badge_properties);
    println!();
    println!(" ┌────────────────────┬──────────┐");
    println!(" │      Coverage      │     %    │");
    println!(" ├────────────────────┼──────────┤");
    println!(" │ Covered regions    │ {:8.4} │", report.regions_percent());
    println!(
        " │ Executed functions │ {:8.4} │",
        report.functions_percent()
    );
    println!(" │ Covered lines      │ {:8.4} │", report.lines_percent());
    println!(" └────────────────────┴──────────┘");
    println!();
    println!(" Badge link: {badge_link}");
    println!();
    Ok(badge_link)
}

fn update_markdown_file(
    badge_link: &str,
    markdown_tag: Option<String>,
    markdown_file: Option<String>,
) -> Result<()> {
    if let Some(markdown_tag) = markdown_tag {
        let markdown_file = markdown_file.unwrap_or_else(|| "README.md".to_string());
        let prefix = format!("[{markdown_tag}]: ");
        let replace = |line: &str| {
            if line.starts_with(&prefix) {
                format!("{prefix}{badge_link}")
            } else {
                line.to_string()
            }
        };
        let content = fs::read_to_string(&markdown_file)
            .map_err(|e| CoverioError::new(e.to_string()))?
            .lines()
            .map(replace)
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(markdown_file, format!("{content}\n"))
            .map_err(|e| CoverioError::new(e.to_string()))?;
    }
    Ok(())
}

/// Runs the application.
///
/// # Errors
///
/// //TODO Add errors description
pub fn run() -> Result<ExitCode> {
    let (app, args) = get_app_and_args();
    match get_action(app, args) {
        CliAction::Run(input_file, badge_properties, markdown_tag, markdown_file) => {
            let content = match input_file {
                Some(file_name) => {
                    if file_name == "-" {
                        get_content_from_stdin()?
                    } else {
                        get_content_from_file(file_name)?
                    }
                }
                None => get_content_from_stdin()?,
            };
            let badge_link = process_content(&content, &badge_properties)?;
            update_markdown_file(&badge_link, markdown_tag, markdown_file)?;
        }
        CliAction::ShowVersion(version) => {
            println!("{version}");
        }
        CliAction::ShowHelp(text) => {
            println!("{text}");
        }
        CliAction::Error(text) => {
            eprintln!("{text}");
            eprintln!(
                "{}",
                get_more_info_hint(ColorMode::default(), &["-h", "--help"])
            );
            return Ok(ExitCode::FAILURE);
        }
    }
    Ok(ExitCode::SUCCESS)
}
