//! Env vars, read once at startup and checked against a schema
//!
//! Each binary has one schema listing every var it reads, e.g. `AxumEnv`.
//! A schema's `load` reads all of its vars before failing, so one run reports every problem.
//! Error messages name the var but never print its value, so secrets stay out of logs.

#[path = "axum.env.rs"]
mod axum;
mod error;
mod field;

pub use axum::AxumEnv;
pub use error::{Error, Issue, Issues};
pub use field::{Field, list, parse};


/// Declares a struct of env vars, plus a `load` function that reads and checks all of them
///
/// Each field gives its type, then a `Field` finished with `.required()`, `.optional()` or `.default(…)`:
///
/// ```ignore
/// env::schema! {
///   pub struct ExampleEnv {
///     /// Port the server listens on
///     pub port: u16 = env::parse("PORT").required(),
///   }
/// }
/// ```
pub use crate::__env_schema as schema;

#[doc(hidden)]
#[macro_export]
macro_rules! __env_schema {
  (
    $(#[$struct_attr:meta])*
    $struct_vis:vis struct $name:ident {
      $(
        $(#[$field_attr:meta])*
        $field_vis:vis $field:ident: $ty:ty = $rule:expr
      ),+ $(,)?
    }
  ) => {
    $(#[$struct_attr])*
    $struct_vis struct $name {
      $(
        $(#[$field_attr])*
        $field_vis $field: $ty,
      )+
    }

    impl $name {
      /// Reads every var, failing with all of their problems at once
      pub fn load() -> ::core::result::Result<Self, $crate::env::Error> {
        let mut issues = $crate::env::Issues::default();
        $(
          let $field: ::core::option::Option<$ty> = issues.take($rule);
        )+

        // Every field is `Some` exactly when no issues were recorded
        match ($($field,)+) {
          ($(::core::option::Option::Some($field),)+) => {
            ::core::result::Result::Ok(Self { $($field),+ })
          }
          _ => ::core::result::Result::Err(issues.into_error()),
        }
      }
    }
  };
}
