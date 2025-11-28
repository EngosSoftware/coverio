#[test]
fn _0001() {
  cli_assert::command!()
    .stdin("")
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: EOF while parsing a value at line 1 column 0\n")
    .execute();
}

#[test]
fn _0002() {
  cli_assert::command!().stdin("[]").failure().code(1).stdout("").stderr("Error: expected object\n").execute();
}

#[test]
fn _0003() {
  cli_assert::command!()
    .stdin("{}")
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected field 'data'\n")
    .execute();
}

#[test]
fn _0004() {
  cli_assert::command!()
    .stdin(r#"{"data":{}}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'data' array\n")
    .execute();
}

#[test]
fn _0005() {
  cli_assert::command!()
    .stdin(r#"{"data":[1,2]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected single element in 'data' array\n")
    .execute();
}

#[test]
fn _0006() {
  cli_assert::command!()
    .stdin(r#"{"data":[1]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected single object in 'data' array\n")
    .execute();
}

#[test]
fn _0007() {
  cli_assert::command!()
    .stdin(r#"{"data":[{}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected field 'totals' in 'data` object\n")
    .execute();
}

#[test]
fn _0008() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":[]}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'totals' object\n")
    .execute();
}

#[test]
fn _0009() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'regions' object\n")
    .execute();
}

#[test]
fn _0010() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{"regions":[]}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'regions' object\n")
    .execute();
}

#[test]
fn _0011() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{"regions":{}}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'count' number\n")
    .execute();
}

#[test]
fn _0012() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{"regions":{"count":"c"}}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'count' number\n")
    .execute();
}

#[test]
fn _0013() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{"regions":{"count":-20}}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: invalid 'count' number\n")
    .execute();
}

#[test]
fn _0014() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{"regions":{"count":20}}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'percent' number\n")
    .execute();
}

#[test]
fn _0015() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{"regions":{"count":80,"percent":"u"}}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'percent' number\n")
    .execute();
}

#[test]
fn _0016() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{"regions":{"count":20,"percent":1.7976931348623157E+309}}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: invalid 'percent' number\n")
    .execute();
}

#[test]
fn _0017() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{"regions":{"count":20,"percent":0.1}}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'functions' object\n")
    .execute();
}

#[test]
fn _0018() {
  cli_assert::command!()
    .stdin(r#"{"data":[{"totals":{"regions":{"count":20,"percent":0.1},"functions":{"count":20,"percent":0.1}}}]}"#)
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: expected 'lines' object\n")
    .execute();
}
