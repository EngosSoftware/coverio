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
    let file = cli_assert::TempFile::new("README.md");
    file.write("line 1\n[cov-badge]: nothing here\nline 3\n");
    cli_assert::command!()
        .current_dir(file.dir())
        .arg("-t")
        .arg("cov-badge")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}

"
        ))
        .stderr("")
        .execute();
    file.assert("line 1\n[cov-badge]: https://img.shields.io/badge/coverage-100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg\nline 3\n");
}

#[test]
fn _0002() {
    let file = cli_assert::TempFile::new("TEXT.md");
    file.write("line 1\n[cov-badge]: nothing here\nline 3\n");
    cli_assert::command!()
        .current_dir(file.dir())
        .arg("-t")
        .arg("cov-badge")
        .arg("-f")
        .arg("TEXT.md")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}

"
        ))
        .stderr("")
        .execute();
    file.assert("line 1\n[cov-badge]: https://img.shields.io/badge/coverage-100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg\nline 3\n");
}

#[test]
fn _0003() {
    #[cfg(not(target_os = "windows"))]
    let expected_stderr = "Error: No such file or directory (os error 2)\n";
    #[cfg(target_os = "windows")]
    let expected_stderr = "Error: The system cannot find the file specified. (os error 2)\n";
    cli_assert::command!()
        .arg("-t")
        .arg("cov-badge")
        .stdin(INPUT)
        .code(1)
        .stderr(expected_stderr)
        .execute();
}

#[test]
fn _0004() {
    let file = cli_assert::TempFile::new("README.md");
    file.write("line 1\n[cov-badge]: nothing here\nline 3\n");
    file.set_readonly(true);
    #[cfg(not(target_os = "windows"))]
    let expected_stderr = "Error: Permission denied (os error 13)\n";
    #[cfg(target_os = "windows")]
    let expected_stderr = "Error: Access is denied. (os error 5)\n";
    cli_assert::command!()
        .current_dir(file.dir())
        .arg("-t")
        .arg("cov-badge")
        .stdin(INPUT)
        .code(1)
        .stderr(expected_stderr)
        .execute();
    file.assert("line 1\n[cov-badge]: nothing here\nline 3\n");
}
