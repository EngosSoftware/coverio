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

 Badge link: https://img.shields.io/badge/coverage-100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg";

#[test]
fn _0001() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}

"#
    ))
    .stderr("");
}

#[test]
fn _0002() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-s")
    .arg("non-existing-style")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}

"#
    ))
    .stderr("");
}

#[test]
fn _0003() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-s")
    .arg("flat")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=flat

"#
    ))
    .stderr("");
}

#[test]
fn _0004() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("--style=flat-square")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=flat-square

"#
    ))
    .stderr("");
}

#[test]
fn _0005() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-s")
    .arg("flat-square")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=flat-square

"#
    ))
    .stderr("");
}

#[test]
fn _0006() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-s")
    .arg("plastic")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=plastic

"#
    ))
    .stderr("");
}

#[test]
fn _0007() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-s")
    .arg("for-the-badge")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=for-the-badge

"#
    ))
    .stderr("");
}

#[test]
fn _0008() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-s")
    .arg("social")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}?style=social

"#
    ))
    .stderr("");
}
