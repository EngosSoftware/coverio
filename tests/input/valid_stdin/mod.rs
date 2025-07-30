use super::*;

const INPUT: &str = r#"
{
  "data": [
    {
      "totals": {
        "functions": {
          "percent": 0.1111
        },
        "lines": {
          "percent": 2.2222
        },
        "regions": {
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

 Badge link: https://img.shields.io/badge/coverage-33%25%20%E2%94%82%200%25%20%E2%94%82%202%25-f52020.svg

";

#[test]
fn _0001() {
  let tc = test_context!();
  tc.command().current_dir(tc.current_dir()).write_stdin(INPUT).assert().success().stdout(EXPECTED).stderr("");
}

#[test]
fn _0002() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-")
    .write_stdin(INPUT)
    .assert()
    .success()
    .stdout(EXPECTED)
    .stderr("");
}
