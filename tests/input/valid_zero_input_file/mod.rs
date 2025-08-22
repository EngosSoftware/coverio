use super::*;

#[test]
fn _0001() {
  let tc = test_context!();
  let expected = r"
 ┌────────────────────┬──────────┐
 │      Coverage      │     %    │
 ├────────────────────┼──────────┤
 │ Covered regions    │   0.0000 │
 │ Executed functions │   0.0000 │
 │ Covered lines      │   0.0000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/coverage-%E2%80%94%20%E2%94%82%20%E2%80%94%20%E2%94%82%20%E2%80%94-21b577.svg

";
  tc.command()
    .current_dir(tc.current_dir())
    .arg("coverage.json")
    .assert()
    .success()
    .stdout(expected)
    .stderr("");
}
