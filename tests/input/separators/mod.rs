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

 Badge link: https://img.shields.io/badge/coverage-";

#[test]
fn _0001() {
    cli_assert::command!()
        .arg("-C=never")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg

"
        ))
        .stderr("")
        .execute();
}

#[test]
fn _0002() {
    let expected_stderr = r"error: invalid value 'non-existing-separator' for '--separator <SEPARATOR>'
  [possible values: space, bar, spaced-bar]

Usage: coverio [OPTIONS]  [ARGS]

For more information, try '-h' or '--help'
Error: kuku
";
    #[cfg(target_os = "windows")]
    let expected_stderr = super::win(expected_stderr);
    cli_assert::command!()
        .arg("-s")
        .arg("non-existing-separator")
        .stdin(INPUT)
        .failure()
        .stdout("")
        .stderr(expected_stderr)
        .execute();
}

#[test]
fn _0003() {
    cli_assert::command!()
        .arg("-C=never")
        .arg("-s")
        .arg("spaced-bar")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg

"
        ))
        .stderr("")
        .execute();
}

#[test]
fn _0004() {
    cli_assert::command!()
        .arg("-C=never")
        .arg("-s")
        .arg("bar")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}100%25%E2%94%82100%25%E2%94%82100%25-21b577.svg

"
        ))
        .stderr("")
        .execute();
}

#[test]
fn _0005() {
    cli_assert::command!()
        .arg("-C=never")
        .arg("-s")
        .arg("space")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}100%25%20100%25%20100%25-21b577.svg

"
        ))
        .stderr("")
        .execute();
}

#[test]
fn _0006() {
    cli_assert::command!()
        .arg("-C=never")
        .arg("--separator")
        .arg("space")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}100%25%20100%25%20100%25-21b577.svg

"
        ))
        .stderr("")
        .execute();
}

#[test]
fn _0007() {
    cli_assert::command!()
        .arg("-C=never")
        .arg("--separator=space")
        .stdin(INPUT)
        .success()
        .stdout(format!(
            r"{EXPECTED}100%25%20100%25%20100%25-21b577.svg

"
        ))
        .stderr("")
        .execute();
}
