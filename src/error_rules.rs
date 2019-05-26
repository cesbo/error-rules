/// Macro for generating error types
///
/// ## Declaring error types
/// Macro `error_rules!` implements `Error`, `Result` types and all necessary traits for `Error`.
/// Arguments should be comma-separated. available next arguments:
///
/// 1. error display text. should be first argument
/// 2. error types for conversions into `Error` chain.
///    By the default implements conversion for: `&str`, `String`
/// 3. custom error types
///
/// Basic usage:
///
/// ```
/// use error_rules::*;
///
/// error_rules! {
///     "app error"
/// }
///
/// assert_eq!(
///     Error::from("error message").to_string().as_str(),
///     "app error => error message");
/// ```
///
/// ## Custom error types
///
/// Custom error without fields:
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     "app error",
///     CustomError => ("custom error"),
/// }
///
/// assert_eq!(
///     Error::from(CustomError).to_string().as_str(),
///     "app error => custom error");
/// ```
///
/// Custom error with fields:
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     "app error",
///     CustomError(usize) => ("custom error value:{}", 0),
/// }
///
/// assert_eq!(
///     Error::from(CustomError(100)).to_string().as_str(),
///     "app error => custom error value:100");
/// ```
///
/// or named fields:
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     "app error",
///     CustomError {
///         value: usize,
///     } => ("custom error value:{}", value),
/// }
///
/// assert_eq!(
///     Error::from(CustomError { value: 100 }).to_string().as_str(),
///     "app error => custom error value:100");
/// ```
#[macro_export]
macro_rules! error_rules {
    () => {};

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

    /* error */

    (
        $text:tt
    ) => {
        #[derive(Debug)]
        pub struct Error(Box<dyn ::std::error::Error>, String);
        pub type Result<T> = ::std::result::Result<T, Error>;

        impl ::std::fmt::Display for Error {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}{} => ", $text, &self.1)?;
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl From<Box<dyn ::std::error::Error>> for Error {
            #[inline]
            fn from(e: Box<dyn ::std::error::Error>) -> Error { Error(e, String::default()) }
        }

        impl ::std::error::Error for Error {
            #[inline]
            fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                Some(self.0.as_ref())
            }
        }

        // Trait for `Result` to convert into `Error` and set error context
        pub trait ResultExt<T, E> {
            fn context<C: ::error_rules::ErrorContext>(self, ctx: &C) -> ::std::result::Result<T, E>;
        }

        impl<T, E: Into<Error>> ResultExt<T, Error> for ::std::result::Result<T, E> {
            fn context<C: ::error_rules::ErrorContext>(self, ctx: &C) -> ::std::result::Result<T, Error> {
                match self {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        let mut e = Into::<Error>::into(e);
                        ctx.context(&mut e.1);
                        Err(e)
                    }
                }
            }
        }

        error_rules! { &str }
        error_rules! { String }
    };

    (
        $text:tt,
        $($tail:tt)*
    ) => {
        error_rules! { $text }
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
///     "app error"
/// }
///
/// fn run() -> Result<()> {
///     bail!("run error");
/// }
///
/// if let Err(e) = run() {
///     println!("{}", e);
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
///     "app error"
/// }
///
/// fn run() -> Result<()> {
///     ensure!(false, "ensure error");
///     Ok(())
/// }
///
/// if let Err(e) = run() {
///     println!("{}", e);
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
