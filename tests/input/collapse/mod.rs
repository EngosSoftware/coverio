use super::{expected, input};

#[test]
fn _0001() {
    cli_assert::command!()
        .arg("-C")
        .arg("never")
        .stdin(input((10, 100.0), (10, 100.0), (10, 100.0)))
        .success()
        .stdout(expected(
            100.0,
            100.0,
            100.0,
            "100%25%20%E2%94%82%20100%25%20%E2%94%82%20100%25-21b577.svg",
        ))
        .stderr("")
        .execute();
}

#[test]
fn _0002() {
    cli_assert::command!()
        .arg("-C")
        .arg("equal")
        .stdin(input((10, 100.0), (10, 100.0), (10, 100.0)))
        .success()
        .stdout(expected(100.0, 100.0, 100.0, "100%25-21b577.svg"))
        .stderr("")
        .execute();
}

#[test]
fn _0003() {
    cli_assert::command!()
        .arg("-C")
        .arg("equal")
        .stdin(input((10, 87.0), (10, 87.0), (10, 87.0)))
        .success()
        .stdout(expected(87.0, 87.0, 87.0, "87%25-21b577.svg"))
        .stderr("")
        .execute();
}

#[test]
fn _0004() {
    cli_assert::command!()
        .arg("--collapse")
        .arg("equal")
        .stdin(input((10, 100.0), (10, 100.0), (10, 100.0)))
        .success()
        .stdout(expected(100.0, 100.0, 100.0, "100%25-21b577.svg"))
        .stderr("")
        .execute();
}

#[test]
fn _0005() {
    cli_assert::command!()
        .arg("-C=functions")
        .stdin(input((30, 97.0), (10, 98.0), (20, 99.0)))
        .success()
        .stdout(expected(97.0, 98.0, 99.0, "98%25-21b577.svg"))
        .stderr("")
        .execute();
}

#[test]
fn _0006() {
    cli_assert::command!()
        .arg("-C=regions")
        .stdin(input((30, 97.0), (10, 98.0), (20, 99.0)))
        .success()
        .stdout(expected(97.0, 98.0, 99.0, "97%25-21b577.svg"))
        .stderr("")
        .execute();
}

#[test]
fn _0007() {
    cli_assert::command!()
        .arg("-C=lines")
        .stdin(input((30, 97.0), (10, 98.0), (20, 99.0)))
        .success()
        .stdout(expected(97.0, 98.0, 99.0, "99%25-21b577.svg"))
        .stderr("")
        .execute();
}
