use super::*;
use assert_fs::prelude::{FileWriteStr, PathAssert};
use std::fs;

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
  let file = assert_fs::NamedTempFile::new("README.md").unwrap();
  file.write_str("line 1\n[cov-badge]: nothing here\nline 3\n").unwrap();
  let path = file.parent().unwrap();
  let tc = test_context!();
  tc.command()
    .current_dir(path)
    .arg("-t")
    .arg("cov-badge")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}

"#
    ))
    .stderr("");
  file.assert("line 1\n[cov-badge]: https://img.shields.io/badge/cov-100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg\nline 3\n");
  file.close().unwrap();
}

#[test]
fn _0002() {
  let file = assert_fs::NamedTempFile::new("TEXT.md").unwrap();
  file.write_str("line 1\n[cov-badge]: nothing here\nline 3\n").unwrap();
  let path = file.parent().unwrap();
  let tc = test_context!();
  tc.command()
    .current_dir(path)
    .arg("-t")
    .arg("cov-badge")
    .arg("-f")
    .arg("TEXT.md")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(format!(
      r#"{EXPECTED}

"#
    ))
    .stderr("");
  file.assert("line 1\n[cov-badge]: https://img.shields.io/badge/cov-100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg\nline 3\n");
  file.close().unwrap();
}

#[test]
fn _0003() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-t")
    .arg("cov-badge")
    .write_stdin(INPUT)
    .assert()
    .code(1)
    .stderr("Error: No such file or directory (os error 2)\n");
}

#[test]
fn _0004() {
  let file = assert_fs::NamedTempFile::new("README.md").unwrap();
  file.write_str("line 1\n[cov-badge]: nothing here\nline 3\n").unwrap();
  let mut perms = fs::metadata(file.path()).unwrap().permissions();
  perms.set_readonly(true);
  fs::set_permissions(file.path(), perms).unwrap();
  let path = file.parent().unwrap();
  let tc = test_context!();
  tc.command()
    .current_dir(path)
    .arg("-t")
    .arg("cov-badge")
    .write_stdin(INPUT)
    .assert()
    .code(1)
    .stderr("Error: Permission denied (os error 13)\n");
  file.assert("line 1\n[cov-badge]: nothing here\nline 3\n");
  file.close().unwrap();
}
