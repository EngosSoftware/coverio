use super::*;

const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "percent": 0.1
        },
        "lines": {
          "percent": 0.1
        },
        "regions": {
          "percent": 0.1
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
 │ Covered regions    │   0.1000 │
 │ Executed functions │   0.1000 │
 │ Covered lines      │   0.1000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/coverage-0%25%20%E2%94%82%200%25%20%E2%94%82%200%25-f52020.svg

";

#[test]
fn _0001() {
  let tc = test_context!();
  tc.command().current_dir(tc.current_dir()).write_stdin(INPUT).assert().success().stdout(EXPECTED).stderr("");
}
