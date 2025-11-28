const INPUT: &[u8] = &[0xff, 0xfe, 0xfd];

#[test]
fn _0001() {
  cli_assert::command!()
    .stdin(INPUT)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: stream did not contain valid UTF-8\n")
    .execute();
}

#[test]
fn _0002() {
  cli_assert::command!()
    .arg("-")
    .stdin(INPUT)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: stream did not contain valid UTF-8\n")
    .execute();
}
