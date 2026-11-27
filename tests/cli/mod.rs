#[test]
fn _0001() {
    let expected_stdout = "coverio v0.1.1\n";
    cli_assert::command!()
        .arg("-V")
        .success()
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}

#[test]
fn _0002() {
    let expected_stdout = r"Better coverage badges for crates

Usage: coverio [OPTIONS]  [ARGS]

Arguments:
  [FILE]  Coverage summary file in JSON format or '-'

Options:
  -s, --style <STYLE>          Badge style [default: flat] [possible values: flat, flat-square, plastic, for-the-badge, social]
  -l, --label <LABEL>          Badge label [default: coverage]
  -r, --separator <SEPARATOR>  Badge label [default: spaced-bar] [possible values: space, bar, spaced-bar]
  -C, --collapse               Collapse to a single value if all are equal
  -n, --no-percent-sign        Hide percent sign in values
  -t, --tag <TAG>              Markdown tag to be updated
  -f, --file <FILE>            Markdown file to be updated
  -V, --version                Print version
  -h, --help                   Print help

";
    cli_assert::command!()
        .arg("-h")
        .success()
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}

#[test]
fn _0003() {
    let expected_stdout = r"Better coverage badges for crates

Usage: coverio [OPTIONS]  [ARGS]

Arguments:
  [FILE]  Coverage summary file in JSON format.
          With no FILE, or when FILE is -, read standard input.
          To generate a coverage summary file, use the following command:
            cargo llvm-cov --json --summary-only

Options:
  -s, --style <STYLE>          Badge style
                                 [default: flat] [possible values: flat, flat-square, plastic, for-the-badge, social]
  -l, --label <LABEL>          Badge label
                                 [default: coverage]
  -r, --separator <SEPARATOR>  Badge label
                                 [default: spaced-bar] [possible values: space, bar, spaced-bar]
  -C, --collapse               Collapse to a single value if all are equal
  -n, --no-percent-sign        Hide percent sign in values
  -t, --tag <TAG>              Markdown tag to be updated
  -f, --file <FILE>            Markdown file to be updated
  -V, --version
  -h, --help                   Print help

";
    cli_assert::command!()
        .arg("--help")
        .success()
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}
