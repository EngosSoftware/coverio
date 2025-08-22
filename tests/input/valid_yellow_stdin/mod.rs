use super::*;

const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "percent": 51.0
        },
        "lines": {
          "percent": 51.0
        },
        "regions": {
          "percent": 51.0
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
 │ Covered regions    │  51.0000 │
 │ Executed functions │  51.0000 │
 │ Covered lines      │  51.0000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/coverage-51%25%20%E2%94%82%2051%25%20%E2%94%82%2051%25-f4b01b.svg

";

#[test]
fn _0001() {
  let tc = test_context!();
  tc.command().current_dir(tc.current_dir()).write_stdin(INPUT).assert().success().stdout(EXPECTED).stderr("");
}
