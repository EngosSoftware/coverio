#[test]
fn _0001() {
    let expected_stdout = "coverio v0.2.0\n";
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
  -S, --style <STYLE>          Badge style [default: flat] [possible values: flat, flat-square, plastic, for-the-badge, social]
  -l, --label <LABEL>          Badge label [default: coverage]
  -s, --separator <SEPARATOR>  Values separator [default: spaced-bar] [possible values: space, bar, spaced-bar]
  -C, --collapse <COLLAPSE>    Collapse to a single value [default: equal] [possible values: never, equal, regions, functions, lines]
  -p, --hide-percent           Hide percent sign in values
  -t, --tag <TAG>              Markdown tag to be updated
  -f, --file <FILE>            Markdown file to be updated
  -V, --version                Print version
  -h, --help                   Print help

";
    #[cfg(target_os = "windows")]
    let expected_stdout = super::win(expected_stdout);
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
  -S, --style <STYLE>          Badge style
                                 [default: flat] [possible values: flat, flat-square, plastic, for-the-badge, social]
  -l, --label <LABEL>          Badge label
                                 [default: coverage]
  -s, --separator <SEPARATOR>  Values separator
                                 [default: spaced-bar] [possible values: space, bar, spaced-bar]
  -C, --collapse <COLLAPSE>    Collapse to a single value
                                 [default: equal] [possible values: never, equal, regions, functions, lines]
  -p, --hide-percent           Hide percent sign in values
  -t, --tag <TAG>              Markdown tag to be updated
  -f, --file <FILE>            Markdown file to be updated
  -V, --version
  -h, --help                   Print help

";
    #[cfg(target_os = "windows")]
    let expected_stdout = super::win(expected_stdout);
    cli_assert::command!()
        .arg("--help")
        .success()
        .stdout(expected_stdout)
        .stderr("")
        .execute();
}
