#[test]
fn _0001() {
  let expected = r"
 ┌────────────────────┬──────────┐
 │      Coverage      │     %    │
 ├────────────────────┼──────────┤
 │ Covered regions    │   0.0000 │
 │ Executed functions │   0.0000 │
 │ Covered lines      │   0.0000 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/cov-%E2%80%94%20%E2%94%82%20%E2%80%94%20%E2%94%82%20%E2%80%94-21b577.svg

";
  cli_assert::command!().arg("coverage.json").success().stdout(expected).stderr("").execute();
}
