use std::{
  fmt::{self, Display},
  str::FromStr,
};


/// A CORS origin: `scheme://host[:port]`, with no path
///
/// One trailing `/` is removed, so a URL like `http://localhost:1420/` works.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Origin(String);

impl Origin {
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl FromStr for Origin {
  type Err = InvalidOrigin;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    let origin = value.strip_suffix('/').unwrap_or(value);
    let (scheme, authority) = origin
      .split_once("://")
      .ok_or(InvalidOrigin)?;

    let valid_scheme = scheme.starts_with(|c: char|
      c.is_ascii_alphabetic()
    ) && scheme.chars().all(|c|
      c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.')
    );
    let valid_authority = !authority.is_empty() 
      && authority.chars().all(|c|
        c.is_ascii_graphic() && !matches!(c, '/' | '?' | '#')
      );

    if valid_scheme && valid_authority {
      Ok(Self(origin.to_owned()))
    } else {
      Err(InvalidOrigin)
    }
  }
}


#[derive(Debug)]
pub struct InvalidOrigin;

impl Display for InvalidOrigin {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("expected scheme://host[:port], e.g. http://localhost:1420")
  }
}

impl std::error::Error for InvalidOrigin {}
