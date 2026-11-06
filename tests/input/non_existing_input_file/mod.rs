#[test]
fn _0001() {
    #[cfg(not(target_os = "windows"))]
    let expected_stderr = "Error: No such file or directory (os error 2)\n";
    #[cfg(target_os = "windows")]
    let expected_stderr = "Error: The system cannot find the file specified. (os error 2)\n";
    cli_assert::command!()
        .arg("coverage.json")
        .failure()
        .code(1)
        .stdout("")
        .stderr(expected_stderr)
        .execute();
}
