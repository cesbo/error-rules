/// Macro for chained error handling
///
/// ## Intro
///
/// Key feature of the `error-rules` crate is chained error handling without pain.
/// For example your application have nested modules: app -> garage -> car -> engine.
/// But how to know where this error happens?
/// Should be saved error context for each module.
/// To do that could be use `.map_err()` before each `?` operator. But this way is too verbose.
/// The `error-rules` macro will do that automaticaly.
/// Idea is simple, each module has own error handler with configurable display text.
/// It pass source error wrapped into own error handler with custom display text.
/// So app will get error with text like: "Garage => Car => Engine => resource temporarily unavailable"
///
/// ## Declaring error types
///
/// Macro `error_rules!` implements `Error`, `Result` types and all necessary traits for `Error`.
/// All arguments should be comma-separated.
///
/// To prevent types shadowing all errors from standard library and other crates should be used
/// with module name. For example: `io::Error`.
///
/// ## Display format
///
/// Error display text defines in tuple after `self =>` keyword.
/// First tuple argument is a format string. Additional arguments:
///
/// - `error` - chained error text
/// - `context` - context for this error
///
/// ```
/// use error_rules::*;
///
/// error_rules! {
///     self => ("app error => {}", error)
/// }
///
/// assert_eq!(
///     Error::from("error message").to_string().as_str(),
///     "app error => error message");
/// ```
///
/// ## Error types
///
/// After display text you could define error types for conversions into `Error` chain.
/// By the default implements conversion for: `&str`, `String`
///
/// ```
/// use std::io;
/// use error_rules::*;
///
/// error_rules! {
///     self => ("app error => {}", error),
///     std::io::Error,
/// }
///
/// let io_error = io::Error::new(io::ErrorKind::Other, "io-error");
/// assert_eq!(
///     Error::from(io_error).to_string().as_str(),
///     "app error => io-error");
/// ```
///
/// ## Custom error types
///
/// Custom errors is an additional error kind to use with `Error`.
/// Defines like `struct` with display arguments after `=>` keyword.
/// Could be defined without fields:
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     self => ("app error => {}", error),
///     CustomError => ("custom error"),
/// }
///
/// assert_eq!(
///     Error::from(CustomError).to_string().as_str(),
///     "app error => custom error");
/// ```
///
/// or with fields:
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     self => ("app error => {}", error),
///     CustomError(usize) => ("custom error value:{}", 0),
/// }
///
/// assert_eq!(
///     Error::from(CustomError(100)).to_string().as_str(),
///     "app error => custom error value:100");
/// ```
///
/// or with named fields:
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     self => ("app error => {}", error),
///     CustomError {
///         value: usize,
///     } => ("custom error value:{}", value),
/// }
///
/// assert_eq!(
///     Error::from(CustomError { value: 100 }).to_string().as_str(),
///     "app error => custom error value:100");
/// ```
///
/// ## Error context
///
/// Error context let to append additional information into error description.
/// For example will be useful to get know which file unavailable on `File::open()`.
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     self => ("file reader ({}) => {}", context, error),
///     std::io::Error,
/// }
///
/// let n = "not-found.txt";
/// let e = std::fs::File::open(n).context(n).unwrap_err();
/// assert_eq!(
///     e.to_string().as_str(),
///     "file reader (not-found.txt) => No such file or directory (os error 2)");
/// ```
#[macro_export]
macro_rules! error_rules {
    () => {};

    /* error */

    (
        self => $display:tt
    ) => {
        #[derive(Debug)]
        pub struct Error {
            error: Box<dyn ::std::error::Error>,
            context: String,
        }
        pub type Result<T> = ::std::result::Result<T, Error>;

        error_rules! { _display Error, $display }

        impl From<Box<dyn ::std::error::Error>> for Error {
            #[inline]
            fn from(e: Box<dyn ::std::error::Error>) -> Error {
                Error {
                    error: e,
                    context: String::default(),
                }
            }
        }

        impl ::std::error::Error for Error {
            #[inline]
            fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                Some(self.error.as_ref())
            }
        }

        // Trait for `Result` to convert into `Error` and set error context
        pub trait ResultExt<T> {
            fn context<S: ToString>(self, ctx: S) -> Result<T>;
        }

        impl<T, E: Into<Error>> ResultExt<T> for ::std::result::Result<T, E> {
            fn context<S: ToString>(self, ctx: S) -> Result<T> {
                match self {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        let mut e = Into::<Error>::into(e);
                        e.context = ctx.to_string();
                        Err(e)
                    }
                }
            }
        }

        error_rules! { &str }
        error_rules! { String }
    };

    (
        self => $display:tt,
        $($tail:tt)*
    ) => {
        error_rules! { self => $display }
        error_rules! { $($tail)* }
    };

    /* display */

    (
        _display $name:ident, ($text:expr)
    ) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, $text)
            }
        }
    };

    (
        _display $name:ident, ($fmt:expr, $($arg:tt),+)
    ) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, $fmt, $(self.$arg),*)
            }
        }
    };

    /* custom errors */

    (
        _error_stuff $name:ident, $display:tt
    ) => {
            error_rules! { _display $name, $display }
            impl ::std::error::Error for $name {}
            error_rules! { $name } /* from */
    };

    (
        $name:ident => $display:tt
    ) => {
        #[derive(Debug)]
        pub struct $name;
        error_rules! { _error_stuff $name, $display }
    };

    (
        $name:ident => $display:tt,
        $($tail:tt)*
    ) => {
        error_rules! { $name => $display }
        error_rules! { $($tail)* }
    };

    (
        $name:ident ( $($field:ty),* ) => $display:tt
    ) => {
        #[derive(Debug)]
        pub struct $name ( $(pub $field),* );
        error_rules! { _error_stuff $name, $display }
    };

    (
        $name:ident ( $($field:ty,)* ) => $display:tt
    ) => {
        error_rules! { $name ( $($field),* ) => $display }
    };

    (
        $name:ident { $($field:ident: $type:ty),* } => $display:tt
    ) => {
        #[derive(Debug)]
        pub struct $name { $(pub $field: $type),* }
        error_rules! { _error_stuff $name, $display }
    };

    (
        $name:ident { $($field:ident: $type:ty,)* } => $display:tt
    ) => {
        error_rules! { $name { $($field: $type),* } => $display }
    };

    (
        $name:ident $fields:tt => $display:tt,
        $($tail:tt)*
    ) => {
        error_rules! { $name $fields => $display }
        error_rules! { $($tail)* }
    };

    /* from */

    (
        $arg:ty
    ) => {
        impl ::std::convert::From<$arg> for Error {
            #[inline]
            fn from(e: $arg) -> Error {
                Error::from(Into::<Box<dyn ::std::error::Error>>::into(e))
            }
        }
    };

    (
        $from:ty,
        $($tail:tt)*
    ) => {
        error_rules! { $from }
        error_rules! { $($tail)* }
    };
}


/// Exits a function and returns Error
///
/// Usage:
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     self => ("{}", error)
/// }
///
/// fn run() -> Result<()> {
///     bail!("bail error");
/// }
///
/// if let Err(e) = run() {
///     assert_eq!(e.to_string().as_str(), "bail error")
/// } else {
///     unreachable!()
/// }
/// ```
#[macro_export]
macro_rules! bail {
    ( $e:expr ) => {
        return Err($e.into())
    };

    ( $fmt:expr, $($arg:tt),+ ) => {
        return Err(format!($fmt, $($arg),+).into())
    };
}


/// Ensure that a boolean expression is true at runtime.
/// If condition is false then invokes `bail!` macro
///
/// /// Usage:
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     self => ("{}", error)
/// }
///
/// fn run() -> Result<()> {
///     ensure!(false, "ensure error");
///     Ok(())
/// }
///
/// if let Err(e) = run() {
///     assert_eq!(e.to_string().as_str(), "ensure error")
/// } else {
///     unreachable!()
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ( $cond:expr, $e:expr ) => {
        if ! ($cond) { bail!( $e ) }
    };

    ( $cond:expr, $fmt:expr, $($arg:tt),* ) => {
        if ! ($cond) { bail!( $fmt, $($arg),* ) }
    };
}
