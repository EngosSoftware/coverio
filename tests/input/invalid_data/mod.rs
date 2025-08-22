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

#[test]
fn _0009() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[{"totals":{}}]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'regions' object\n");
}

#[test]
fn _0010() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[{"totals":{"regions":[]}}]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'regions' object\n");
}

#[test]
fn _0011() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[{"totals":{"regions":{}}}]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'percent' number\n");
}

#[test]
fn _0012() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[{"totals":{"regions":{"percent":"u"}}}]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'percent' number\n");
}

#[test]
fn _0013() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[{"totals":{"regions":{"percent":1.7976931348623157E+309}}}]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: invalid 'percent' number\n");
}

#[test]
fn _0014() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[{"totals":{"regions":{"percent":0.1}}}]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'functions' object\n");
}

#[test]
fn _0015() {
  let tc = test_context!();
  tc.command()
    .current_dir(tc.current_dir())
    .write_stdin(r#"{"data":[{"totals":{"regions":{"percent":0.1},"functions":{"percent":0.1}}}]}"#)
    .assert()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'lines' object\n");
}
