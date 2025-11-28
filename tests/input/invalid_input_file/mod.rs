#[test]
fn _0001() {
  cli_assert::command!()
    .arg("coverage.json")
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: EOF while parsing an object at line 2 column 0\n")
    .execute();
}
