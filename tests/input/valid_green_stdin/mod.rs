use super::*;

const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "count": 10,
          "percent": 80.1
        },
        "lines": {
          "count": 10,
          "percent": 80.1
        },
        "regions": {
          "count": 10,
          "percent": 80.1
        }
      }
    }
  ]
}
"#;

const EXPECTED: &str = r"
 ┌────────────────────┬──────────┐
 │      Coverage      │     %    │
 ├────────────────────┼──────────┤
 │ Covered regions    │  80.1000 │
 │ Executed functions │  80.1000 │
 │ Covered lines      │  80.1000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/coverage-80%25%20%E2%94%82%2080%25%20%E2%94%82%2080%25-21b577.svg

";

#[test]
fn _0001() {
  let tc = test_context!();
  tc.command().current_dir(tc.current_dir()).write_stdin(INPUT).assert().success().stdout(EXPECTED).stderr("");
}
