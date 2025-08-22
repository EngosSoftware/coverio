//! # Definition of the result and error types

use std::fmt;

/// Result type definition.
pub type Result<T, E = CoverioError> = std::result::Result<T, E>;

/// Error definition.
#[derive(PartialEq, Eq)]
pub struct CoverioError(String);

impl fmt::Debug for CoverioError {
  /// Implementation of [Debug] trait for [CoverioError].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl CoverioError {
  /// Creates a new [CoverioError].
  pub fn new(message: impl AsRef<str>) -> Self {
    Self(message.as_ref().into())
  }
}
