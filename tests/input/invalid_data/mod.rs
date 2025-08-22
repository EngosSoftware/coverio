use super::*;

#[test]
fn _0001() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin("")
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: EOF while parsing a value at line 1 column 0\n");
}

#[test]
fn _0002() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin("[]")
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected object\n");
}

#[test]
fn _0003() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin("{}")
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected field 'data'\n");
}

#[test]
fn _0004() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":{}}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'data' array\n");
}

#[test]
fn _0005() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[1,2]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected single element in 'data' array\n");
}

#[test]
fn _0006() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[1]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected single object in 'data' array\n");
}

#[test]
fn _0007() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[{}]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected field 'totals' in 'data` object\n");
}

#[test]
fn _0008() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[{"totals":[]}]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'totals' object\n");
}
