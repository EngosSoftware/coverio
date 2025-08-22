use super::*;

const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "count": 0,
          "percent": 0
        },
        "lines": {
          "count": 10,
          "percent": 55.5555
        },
        "regions": {
          "count": 0,
          "percent": 0
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
 │ Covered regions    │   0.0000 │
 │ Executed functions │   0.0000 │
 │ Covered lines      │  55.5555 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/coverage-%E2%80%94%20%E2%94%82%20%E2%80%94%20%E2%94%82%2055%25-f4b01b.svg

";

#[test]
fn _0001() {
  let tc = test_context!();
  tc.command().current_dir(tc.current_dir()).write_stdin(INPUT).assert().success().stdout(EXPECTED).stderr("");
}
