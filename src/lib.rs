//! ## Intro
//!
//! Key feature of the `error-rules` is chained error handling.
//!
//! Idea is simple, each module has own error handler.
//! Source error wrapped into error handler with configurable display text.
//!
//! ## Declaring error types
//!
//! Macro `error_rules!` implements `Error`, `Result` types and all necessary traits for `Error`.
//! All arguments should be comma-separated.
//!
//! To prevent types shadowing all errors from standard library and other crates should be used
//! with module name. For example: `io::Error`.
//!
//! ## Display format
//!
//! Error display text defines in tuple after `Error =>` keyword.
//! First tuple argument is a format string. Additional arguments:
//!
//! - `error` - chained error text
//!
//! ```
//! use error_rules::*;
//!
//! error_rules! {
//!     Error => ("app error => {}", error)
//! }
//!
//! assert_eq!(
//!     Error::from("error message").to_string().as_str(),
//!     "app error => error message");
//! ```
//!
//! ## Error types
//!
//! After display text you could define error types for conversions into `Error` chain.
//! By the default implements conversion for: `&str`, `String`
//!
//! ```
//! use std::io;
//! use error_rules::*;
//!
//! error_rules! {
//!     Error => ("app error => {}", error),
//!     std::io::Error,
//! }
//!
//! let io_error = io::Error::new(io::ErrorKind::Other, "io-error");
//! assert_eq!(
//!     Error::from(io_error).to_string().as_str(),
//!     "app error => io-error");
//! ```
//!
//! ## Custom error types
//!
//! Custom errors is an additional error kind to use with `Error`.
//! Defines like `struct` with display arguments after `=>` keyword.
//! Could be defined without fields:
//!
//! ```
//! # use error_rules::*;
//! error_rules! {
//!     Error => ("app error => {}", error),
//!     CustomError => ("custom error"),
//! }
//!
//! assert_eq!(
//!     Error::from(CustomError).to_string().as_str(),
//!     "app error => custom error");
//! ```
//!
//! or with fields:
//!
//! ```
//! # use error_rules::*;
//! error_rules! {
//!     Error => ("app error => {}", error),
//!     CustomError(usize) => ("custom error value:{}", 0),
//! }
//!
//! assert_eq!(
//!     Error::from(CustomError(100)).to_string().as_str(),
//!     "app error => custom error value:100");
//! ```
//!
//! or with named fields:
//!
//! ```
//! # use error_rules::*;
//! error_rules! {
//!     Error => ("app error => {}", error),
//!     CustomError {
//!         value: usize,
//!     } => ("custom error value:{}", value),
//! }
//!
//! assert_eq!(
//!     Error::from(CustomError { value: 100 }).to_string().as_str(),
//!     "app error => custom error value:100");
//! ```

#[macro_use]
mod error_rules;
