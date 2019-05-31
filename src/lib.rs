//! ## Into
//!
//! error-rules is a derive macro to implement error handler based on the enum.
//!
//! Implements next interfaces:
//!
//! - `std::fmt::Display` for all enum variants
//! - `std::error::Error` and `std::convert::From` for source errors
//!
//! ## Declaring error
//!
//! ```rust
//! use std::io;
//! use error_rules::*;
//!
//! #[derive(Debug, Error)]
//! enum HumanError {
//!     #[error_from("Human IO => {}", 0)]
//!     Io(io::Error),
//!     #[error_kind("Human => not found")]
//!     NotFound,
//! }
//!
//! #[derive(Default)]
//! struct Human;
//!
//! impl Human {
//!     pub fn invoke_failure(&self) -> Result<(), HumanError> {
//!         Err(io::Error::from(io::ErrorKind::PermissionDenied))?;
//!         unreachable!()
//!     }
//! }
//!
//! #[derive(Debug, Error)]
//! enum BikeError {
//!     #[error_from("Bike IO => {}", 0)]
//!     Io(io::Error),
//!     #[error_from("Bike => {}", 0)]
//!     Human(HumanError),
//!     #[error_kind("Bike => speed limit")]
//!     SpeedLimit,
//! }
//!
//! #[derive(Default)]
//! struct Bike(Human);
//!
//! impl Bike {
//!     pub fn chained_failure(&self) -> Result<(), BikeError> {
//!         self.0.invoke_failure()?;
//!         unreachable!()
//!     }
//!
//!     pub fn invoke_failure(&self) -> Result<(), BikeError> {
//!         Err(BikeError::SpeedLimit)?;
//!         unreachable!()
//!     }
//! }
//!
//! let b = Bike::default();
//!
//! let error = b.chained_failure().unwrap_err();
//! assert_eq!(error.to_string().as_str(),
//!     "Bike => Human IO => permission denied");
//!
//! let error = b.invoke_failure().unwrap_err();
//! assert_eq!(error.to_string().as_str(),
//!     "Bike => speed limit");
//! ```
//!
//! ## error_from
//!
//! error_from attribute implements a converter from any error type into `Error`.
//! Converted type should implements `std::error::Error` itnerface.
//!
//! ## error_kind
//!
//! error_kind attribute describes additional variant for `Error`.
//! Could be defined without fields or with fields tuple
//!
//! ## display attributes
//!
//! `error_from` and `error_kind` contain list of attributes to display error.
//! First attribute should be literal string. Other attributes is a number of the
//! unnamed field in the tuple. Started from 0.

pub use error_derive::*;
