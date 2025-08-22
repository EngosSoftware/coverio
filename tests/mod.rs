use assert_cmd::Command;
use std::path::PathBuf;

mod input;

struct TestContext {
  current_dir: PathBuf,
}

impl TestContext {
  /// Returns the default command.
  pub fn command(&self) -> Command {
    Command::cargo_bin("coverio").unwrap()
  }

  /// Returns the relative path to current directory.
  pub fn current_dir(&self) -> &PathBuf {
    &self.current_dir
  }
}

macro_rules! test_context {
  () => {{
    let file_name = file!();
    let current_dir = std::path::Path::new(file_name).parent().unwrap().to_path_buf();
    TestContext { current_dir }
  }};
}

use test_context;
