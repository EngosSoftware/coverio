mod report;

use report::CoverageReport;
use serde_json::Value;
use std::{env, fs};

fn main() {
  let args = env::args().collect::<Vec<String>>();
  if args.len() == 2 {
    let content = fs::read_to_string(&args[1]).expect("failed to read input file");
    let json: Value = serde_json::from_str(&content).expect("failed to parse input JSON");
    let mut report = CoverageReport::new();
    report.analyse(&json);
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
}
