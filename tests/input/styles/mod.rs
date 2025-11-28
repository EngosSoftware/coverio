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

 Badge link: https://img.shields.io/badge/cov-100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg";

#[test]
fn _0001() {
  cli_assert::command!()
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0002() {
  cli_assert::command!()
    .arg("-s")
    .arg("non-existing-style")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0003() {
  cli_assert::command!()
    .arg("-s")
    .arg("flat")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0004() {
  cli_assert::command!()
    .arg("--style=flat-square")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=flat-square

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0005() {
  cli_assert::command!()
    .arg("-s")
    .arg("flat-square")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=flat-square

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0006() {
  cli_assert::command!()
    .arg("-s")
    .arg("plastic")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=plastic

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0007() {
  cli_assert::command!()
    .arg("-s")
    .arg("for-the-badge")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=for-the-badge

"#
    ))
    .stderr("")
    .execute();
}

#[test]
fn _0008() {
  cli_assert::command!()
    .arg("-s")
    .arg("social")
    .stdin(INPUT)
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=social

"#
    ))
    .stderr("")
    .execute();
}
