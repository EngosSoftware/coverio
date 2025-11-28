#[test]
fn _0001() {
  let expected = r"
 ┌────────────────────┬──────────┐
 │      Coverage      │     %    │
 ├────────────────────┼──────────┤
 │ Covered regions    │  94.6823 │
 │ Executed functions │  93.6416 │
 │ Covered lines      │  95.8957 │
 └────────────────────┴──────────┘

 Badge link: https://img.shields.io/badge/cov-94%25%20%E2%94%82%2093%25%20%E2%94%82%2095%25-21b577.svg

";
  cli_assert::command!().arg("coverage.json").success().stdout(expected).stderr("").execute();
}
