const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "count": 10,
          "percent": 51.1
        },
        "lines": {
          "count": 10,
          "percent": 51.2
        },
        "regions": {
          "count": 10,
          "percent": 51.3
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
 │ Covered regions    │  51.3000 │
 │ Executed functions │  51.1000 │
 │ Covered lines      │  51.2000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/coverage-51%25%20%E2%94%82%2051%25%20%E2%94%82%2051%25-f4b01b.svg

";

#[test]
fn _0001() {
    cli_assert::command!()
        .stdin(INPUT)
        .success()
        .stdout(EXPECTED)
        .stderr("")
        .execute();
}
