/// Macro for chained error handling
#[macro_export]
macro_rules! error_rules {
    () => {};

    /* error */

    (
        Error => $display:tt
    ) => {
        #[derive(Debug)]
        pub struct Error {
            error: Box<dyn ::std::error::Error>,
        }
        pub type Result<T> = ::std::result::Result<T, Error>;

        error_rules! { _display Error, $display }

        impl From<Box<dyn ::std::error::Error>> for Error {
            #[inline]
            fn from(e: Box<dyn ::std::error::Error>) -> Error {
                Error {
                    error: e,
                }
            }
        }

        impl ::std::error::Error for Error {
            #[inline]
            fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                Some(self.error.as_ref())
            }
        }

        error_rules! { &str }
        error_rules! { String }
    };

    (
        Error => $display:tt,
        $($tail:tt)*
    ) => {
        error_rules! { Error => $display }
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
///     Error => ("{}", error)
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
/// Usage:
///
/// ```
/// # use error_rules::*;
/// error_rules! {
///     Error => ("{}", error)
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
