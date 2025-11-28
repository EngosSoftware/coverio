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
  cli_assert::command!()
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0002() {
  cli_assert::command!()
    .arg("-r")
    .arg("non-existing-separator")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0003() {
  cli_assert::command!()
    .arg("-r")
    .arg("spaced-bar")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0004() {
  cli_assert::command!()
    .arg("-r")
    .arg("bar")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%25%E2%94%82100%25%E2%94%82100%25-21b577.svg

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0005() {
  cli_assert::command!()
    .arg("-r")
    .arg("space")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%25%20100%25%20100%25-21b577.svg

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0006() {
  cli_assert::command!()
    .arg("--separator")
    .arg("space")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%25%20100%25%20100%25-21b577.svg

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0007() {
  cli_assert::command!()
    .arg("--separator=space")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}100%25%20100%25%20100%25-21b577.svg

"#
    ))
    .stderr("")
    .execute();
}
