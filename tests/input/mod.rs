mod collapse;
mod invalid_data;
mod invalid_input_file;
mod invalid_stdin;
mod no_percent_sign;
mod non_existing_input_file;
mod replacement;
mod separators;
mod styles;
mod valid_green_stdin;
mod valid_input_file;
mod valid_label;
mod valid_red_stdin;
mod valid_stdin;
mod valid_yellow_stdin;
mod valid_zero_green_stdin;
mod valid_zero_input_file;
mod valid_zero_red_stdin;
mod valid_zero_yellow_stdin;

/// Utility function for building JSON input .
fn input(regions: (usize, f64), functions: (usize, f64), lines: (usize, f64)) -> String {
    format!(
        r#"
{{
  "data": [
    {{
      "totals": {{
        "functions": {{
          "count": {},
          "percent": {}
        }},
        "lines": {{
          "count": {},
          "percent": {}
        }},
        "regions": {{
          "count": {},
          "percent": {}
        }}
      }}
    }}
  ]
}}
"#,
        functions.0, functions.1, lines.0, lines.1, regions.0, regions.1
    )
}

/// Utility function for building expected report summary.
fn expected(regions: f64, functions: f64, lines: f64, suffix: &str) -> String {
    format!(
        r"
 ┌────────────────────┬──────────┐
 │      Coverage      │     %    │
 ├────────────────────┼──────────┤
 │ Covered regions    │ {regions:8.4} │
 │ Executed functions │ {functions:8.4} │
 │ Covered lines      │ {lines:8.4} │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/coverage-{suffix}

",
    )
}
