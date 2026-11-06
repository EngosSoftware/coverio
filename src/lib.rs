#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../docs/lib.rs.md")]

mod cli;
mod errors;
mod report;

pub use errors::{CoverioError, Result};

use report::{BadgeProperties, CoverageReport};
use std::io::Read;
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
        let content = fs::read_to_string(markdown_file.clone())
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
pub fn run() -> Result<()> {
    let matches = cli::get_command().get_matches();
    let input_file = cli::input_file(&matches);
    let badge_properties = BadgeProperties::new()
        .with_badge_style(cli::badge_style(&matches))
        .with_badge_label(cli::badge_label(&matches))
        .with_separator_style(cli::separator_style(&matches))
        .with_no_percent_sign(cli::no_percent_sign(&matches))
        .with_squash(cli::collapse(&matches));
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
    let markdown_tag = cli::markdown_tag(&matches);
    let markdown_file = cli::markdown_file(&matches);
    update_markdown_file(&badge_link, markdown_tag, markdown_file)?;
    Ok(())
}
