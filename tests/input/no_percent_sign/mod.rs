const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "count": 10,
          "percent": 99.0
        },
        "lines": {
          "count": 10,
          "percent": 98.0
        },
        "regions": {
          "count": 10,
          "percent": 100.0
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
 │ Covered regions    │ 100.0000 │
 │ Executed functions │  99.0000 │
 │ Covered lines      │  98.0000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/coverage-";

#[test]
fn _0001() {
    cli_assert::command!()
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}100%25%20%E2%94%82%2099%25%20%E2%94%82%2098%25-21b577.svg

"
        ))
        .stderr("")
        .execute();
}

#[test]
fn _0002() {
    cli_assert::command!()
        .arg("-n")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}100%20%E2%94%82%2099%20%E2%94%82%2098-21b577.svg

"
        ))
        .stderr("")
        .execute();
}

#[test]
fn _0003() {
    cli_assert::command!()
        .arg("--no-percent-sign")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}100%20%E2%94%82%2099%20%E2%94%82%2098-21b577.svg

"
        ))
        .stderr("")
        .execute();
}
