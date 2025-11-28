const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "count": 10,
          "percent": 0.1111
        },
        "lines": {
          "count": 10,
          "percent": 2.2222
        },
        "regions": {
          "count": 10,
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
 │ Executed functions │   0.1111 │
 │ Covered lines      │   2.2222 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/cov-33%25%20%E2%94%82%200%25%20%E2%94%82%202%25-f52020.svg

";

#[test]
fn _0001() {
  cli_assert::command!().stdin(INPUT).success().stdout(EXPECTED).stderr("").execute();
}

#[test]
fn _0002() {
  cli_assert::command!().arg("-").stdin(INPUT).success().stdout(EXPECTED).stderr("").execute();
}
