use super::*;

#[test]
fn _0001() {
  let tc = test_context!();
  let expected = r"
┌────────────────────┬──────────┐
│      Coverage      │     %    │
├────────────────────┼──────────┤
│ Covered regions    │  94.6823 │
│ Executed functions │  93.6416 │
│ Covered lines      │  95.8957 │
└────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/coverage-94%25%20%E2%94%82%2093%25%20%E2%94%82%2095%25-21b577.svg

";
  tc.command()
    .current_dir(tc.current_dir())
    .arg("coverage.json")
    .assert()
    .success()
    .stdout(expected)
    .stderr("");
}
