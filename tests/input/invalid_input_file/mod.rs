use super::*;

#[test]
fn _0001() {
  let tc = test_context!();

  tc.command()
    .current_dir(tc.current_dir())
    .arg("coverage.json")
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: EOF while parsing an object at line 2 column 0\n");
}
