#[test]
fn _0001() {
  cli_assert::command!()
    .arg("coverage.json")
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: No such file or directory (os error 2)\n")
    .execute();
}
