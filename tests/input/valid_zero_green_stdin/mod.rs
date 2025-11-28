const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "count": 10,
          "percent": 88.8888
        },
        "lines": {
          "count": 0,
          "percent": 0
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
 │ Executed functions │  88.8888 │
 │ Covered lines      │   0.0000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/cov-%E2%80%94%20%E2%94%82%2088%25%20%E2%94%82%20%E2%80%94-21b577.svg

";

#[test]
fn _0001() {
  cli_assert::command!().stdin(INPUT).success().stdout(EXPECTED).stderr("").execute();
}
