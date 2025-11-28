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
          "count": 0,
          "percent": 0
        },
        "regions": {
          "count": 1,
          "percent": 33.3333
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
 │ Covered regions    │  33.3333 │
 │ Executed functions │   0.0000 │
 │ Covered lines      │   0.0000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/cov-33%25%20%E2%94%82%20%E2%80%94%20%E2%94%82%20%E2%80%94-f52020.svg

";

#[test]
fn _0001() {
  cli_assert::command!().stdin(INPUT).success().stdout(EXPECTED).stderr("").execute();
}
