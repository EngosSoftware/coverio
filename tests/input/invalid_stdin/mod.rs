use super::*;

const INPUT: &[u8] = &[0xff, 0xfe, 0xfd];

#[test]
fn _0001() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(INPUT)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: stream did not contain valid UTF-8\n");
}

#[test]
fn _0002() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-")
    .write_stdin(INPUT)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: stream did not contain valid UTF-8\n");
}
