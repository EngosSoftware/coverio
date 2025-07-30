mod cli;
mod report;

use report::CoverageReport;
use std::io::Read;
use std::{fs, io};

fn get_content_from_file(file_name: &str) -> String {
  fs::read_to_string(file_name).expect("failed to read from input file")
}

fn get_content_from_stdin() -> String {
  let mut content = String::new();
  io::stdin().read_to_string(&mut content).expect("failed to read from stdin");
  content
}

fn process_content(content: String) {
  let json: serde_json::Value = serde_json::from_str(&content).expect("failed to parse input JSON");
  let mut report = CoverageReport::new();
  report.analyze(&json);
  println!();
  println!(" ┌────────────────────┬──────────┐");
  println!(" │      Coverage      │     %    │");
  println!(" ├────────────────────┼──────────┤");
  println!(" │ Covered regions    │ {:8.4} │", report.regions_percent());
  println!(" │ Executed functions │ {:8.4} │", report.functions_percent());
  println!(" │ Covered lines      │ {:8.4} │", report.lines_percent());
  println!(" └────────────────────┴──────────┘");
  println!();
  println!(" Badge link: {}", report.badge());
  println!();
}

fn main() {
  let matches = cli::get_command().get_matches();
  let input_file = cli::input_file(&matches);
  let content = match input_file {
    Some(file_name) => {
      if file_name == "-" {
        get_content_from_stdin()
      } else {
        get_content_from_file(file_name)
      }
    }
    None => get_content_from_stdin(),
  };
  process_content(content);
}
