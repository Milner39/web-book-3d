use std::{
  any::type_name,
  env::{self, VarError},
  fmt::Display,
  str::FromStr,
};

use super::Issue;



/// What state a named env var is
enum Raw {
  /// Var is unset
  Unset,

  /// Var is set, but empty or only whitespace
  Empty,

  /// Var is set
  Set(String),
}


/// An env var and how to parse its value
///
/// Finish it with `.required()`, `.optional()` or `.default(…)` to choose what
/// an unset var means.
#[must_use = "finish the field with `.required()`, `.optional()` or `.default(…)`"]
pub struct Field<T> {
  name: &'static str,
  parser: fn(&str) -> Result<T, String>,
}

impl<T> Field<T> {
  /// The var must be set to a valid value
  pub fn required(self) -> Result<T, Issue> {
    match self.raw()? {
      Raw::Set(value) => self.parse(&value),
      Raw::Unset => Err(Issue::new(self.name, "not set")),
      Raw::Empty => Err(Issue::new(self.name, "set but empty")),
    }
  }

  /// An unset or empty var gives `None`
  pub fn optional(self) -> Result<Option<T>, Issue> {
    match self.raw()? {
      Raw::Set(value) => self.parse(&value).map(Some),
      Raw::Unset | Raw::Empty => Ok(None),
    }
  }

  /// An unset or empty var gives `value`
  pub fn default(self, value: T) -> Result<T, Issue> {
    Ok(self.optional()?.unwrap_or(value))
  }

  fn raw(&self) -> Result<Raw, Issue> {
    match env::var(self.name) {
      Ok(value) if value.trim().is_empty() => Ok(Raw::Empty),
      Ok(value) => Ok(Raw::Set(value)),
      Err(VarError::NotPresent) => Ok(Raw::Unset),
      Err(VarError::NotUnicode(_)) => Err(
        Issue::new(self.name, "not valid UTF-8")
      ),
    }
  }

  fn parse(&self, value: &str) -> Result<T, Issue> {
    (self.parser)(value.trim()).map_err(|problem|
      Issue::new(self.name, problem)
    )
  }
}


/// Any type that implements `FromStr`, e.g. `SocketAddr`
///
/// The type's parse error is shown after the var's name, so it must not echo
/// the value back.
pub fn parse<T: FromStr<Err: Display>>(name: &'static str) -> Field<T> {
  Field {
    name,
    parser: parse_as::<T>,
  }
}

/// A comma-separated list of any `FromStr` type. Items are trimmed and empty
/// items are dropped
pub fn list<T: FromStr<Err: Display>>(name: &'static str) -> Field<Vec<T>> {
  Field {
    name,
    parser: |value| {
      value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .enumerate()
        .map(|(index, item)| {
          parse_as::<T>(item).map_err(|problem|
            format!("item {} is {problem}", index + 1)
          )
        })
        .collect()
    },
  }
}


/// Parses one value, naming the expected type if it fails
fn parse_as<T: FromStr<Err: Display>>(value: &str) -> Result<T, String> {
  value
    .parse()
    .map_err(|err|
      format!("not a valid {} ({err})", short_type_name::<T>())
    )
}

/// `core::net::SocketAddr` -> `SocketAddr`, to keep messages short
fn short_type_name<T>() -> &'static str {
  let path = type_name::<T>().split('<').next().unwrap_or_default();
  path.rsplit("::").next().unwrap_or_default()
}
