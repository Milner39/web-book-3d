use std::fmt::{self, Display};


/// One problem with one env var
#[derive(Debug)]
pub struct Issue {
  name: &'static str,
  problem: String,
}

impl Issue {
  pub(crate) fn new(name: &'static str, problem: impl Into<String>) -> Self {
    Self {
      name,
      problem: problem.into(),
    }
  }
}

impl Display for Issue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.name, self.problem)
  }
}



/// Collects every issue while a schema loads, and becomes an `Error` if there
/// are any.
#[doc(hidden)]
#[derive(Default)]
pub struct Issues(Vec<Issue>);

impl Issues {
  /// Returns the value, or records the issue and returns `None`
  pub fn take<T>(&mut self, result: Result<T, Issue>) -> Option<T> {
    match result {
      Ok(value) => Some(value),
      Err(issue) => {self.0.push(issue); None}
    }
  }

  pub fn into_error(self) -> Error {
    Error { issues: self.0 }
  }
}



/// What a schema's `load` returns when any env var is missing or invalid
///
/// Holds every issue found, so all of them are reported.
#[derive(Debug)]
pub struct Error {
  issues: Vec<Issue>,
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let count = self.issues.len();
    let plural = if count == 1 { "" } else { "s" };
    write!(f, "invalid environment ({count} problem{plural}):")?;

    for issue in &self.issues {
      write!(f, "\n  {issue}")?;
    }
    Ok(())
  }
}

impl std::error::Error for Error {}
