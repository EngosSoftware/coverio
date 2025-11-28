const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "count": 10,
          "percent": 0.1
        },
        "lines": {
          "count": 10,
          "percent": 0.1
        },
        "regions": {
          "count": 10,
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

 Badge link: https://img.shields.io/badge/cov-0%25%20%E2%94%82%200%25%20%E2%94%82%200%25-f52020.svg

";

#[test]
fn _0001() {
  cli_assert::command!().stdin(INPUT).success().stdout(EXPECTED).stderr("").execute();
}
