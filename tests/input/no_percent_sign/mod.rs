use super::*;

const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "count": 10,
          "percent": 100.0
        },
        "lines": {
          "count": 10,
          "percent": 100.0
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
 │ Executed functions │ 100.0000 │
 │ Covered lines      │ 100.0000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/cov-";

#[test]
fn _0001() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg

"#
    ))
    .stderr("");
}

#[test]
fn _0002() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-n")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%20%E2%94%82%20100%20%E2%94%82%20100-21b577.svg

"#
    ))
    .stderr("");
}

#[test]
fn _0003() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("--no-percent-sign")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%20%E2%94%82%20100%20%E2%94%82%20100-21b577.svg

"#
    ))
    .stderr("");
}
