mod cli;
mod errors;
mod report;

use crate::report::BadgeProperties;
use errors::{CoverioError, Result};
use report::CoverageReport;
use std::io::Read;
use std::{fs, io};

fn get_content_from_file(file_name: String) -> Result<String, CoverioError> {
  fs::read_to_string(file_name).map_err(|e| CoverioError::new(e.to_string()))
}

fn get_content_from_stdin() -> Result<String, CoverioError> {
  let mut content = String::new();
  io::stdin().read_to_string(&mut content).map_err(|e| CoverioError::new(e.to_string()))?;
  Ok(content)
}

fn process_content(content: String, badge_properties: BadgeProperties) -> Result<String, CoverioError> {
  let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| CoverioError::new(e.to_string()))?;
  let mut report = CoverageReport::new();
  report.analyze(&json)?;
  let badge_link = report.badge(badge_properties);
  println!();
  println!(" ┌────────────────────┬──────────┐");
  println!(" │      Coverage      │     %    │");
  println!(" ├────────────────────┼──────────┤");
  println!(" │ Covered regions    │ {:8.4} │", report.regions_percent());
  println!(" │ Executed functions │ {:8.4} │", report.functions_percent());
  println!(" │ Covered lines      │ {:8.4} │", report.lines_percent());
  println!(" └────────────────────┴──────────┘");
  println!();
  println!(" Badge link: {}", badge_link);
  println!();
  Ok(badge_link)
}

fn update_markdown_file(badge_link: String, markdown_tag: Option<String>, markdown_file: Option<String>) -> Result<(), CoverioError> {
  if let Some(markdown_tag) = markdown_tag {
    let markdown_file = markdown_file.unwrap_or("README.md".to_string());
    let prefix = format!("[{}]: ", markdown_tag);
    let replace = |line: &str| {
      if line.starts_with(&prefix) {
        format!("{}{}", prefix, badge_link)
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
    fs::write(markdown_file, format!("{}\n", content)).map_err(|e| CoverioError::new(e.to_string()))?;
  }
  Ok(())
}

fn main() -> Result<(), CoverioError> {
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
  let badge_link = process_content(content, badge_properties)?;
  let markdown_tag = cli::markdown_tag(&matches);
  let markdown_file = cli::markdown_file(&matches);
  update_markdown_file(badge_link, markdown_tag, markdown_file)?;
  Ok(())
}
