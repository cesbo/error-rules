//! ## Intro
//!
//! Key feature of the `error-rules` crate is chained error handling without pain.
//! For example your application have nested modules: app -> garage -> car -> engine.
//! But how to know where this error happens?
//! Should be saved error context for each module.
//! To do that could be use `.map_err()` before each `?` operator. But this way is too verbose.
//! The `error-rules` macro will do that automaticaly.
//! Idea is simple, each module has own error handler with configurable display text.
//! It pass source error wrapped into own error handler with custom display text.
//! So app will get error with text like: "Garage => Car => Engine => resource temporarily unavailable"
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
//! Error display text defines in tuple after `self =>` keyword.
//! First tuple argument is a format string. Additional arguments:
//!
//! - `error` - chained error text
//! - `context` - context for this error
//!
//! ```
//! use error_rules::*;
//!
//! error_rules! {
//!     self => ("app error => {}", error)
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
//!     self => ("app error => {}", error),
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
//!     self => ("app error => {}", error),
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
//!     self => ("app error => {}", error),
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
//!     self => ("app error => {}", error),
//!     CustomError {
//!         value: usize,
//!     } => ("custom error value:{}", value),
//! }
//!
//! assert_eq!(
//!     Error::from(CustomError { value: 100 }).to_string().as_str(),
//!     "app error => custom error value:100");
//! ```
//!
//! ## Error context
//!
//! Error context let to append additional information into error description.
//! For example will be useful to get know which file unavailable on `File::open()`.
//!
//! ```
//! # use error_rules::*;
//! error_rules! {
//!     self => ("file reader ({}) => {}", context, error),
//!     std::io::Error,
//! }
//!
//! let n = "not-found.txt";
//! let e = std::fs::File::open(n).context(n).unwrap_err();
//! assert_eq!(
//!     e.to_string().as_str(),
//!     "file reader (not-found.txt) => No such file or directory (os error 2)");
//! ```

#[macro_use]
mod error_rules;
