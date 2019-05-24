/// Macro for generating error types
///
/// Implements `Error`, `Result` types and all necessary traits for `Error`
///
/// Parts of the `error_rules!` macro:
///
/// - `name` - defining errors name for the error chaining. Will be a part of resulted string
/// - `from` - defining types for conversion into `Error`.
///            By the default implements conversion for `&str`, `String`.
///            Types should be comma-separated
/// - `errors` - defining custom error types
///
/// Usage:
///
/// ```
/// #[macro_use]
/// extern crate error_rules;
///
/// error_rules! {
///     name = "App"
///     from { std::io::Error }
///     errors {
///         CustomError => ("custom error")
///     }
/// }
///
/// fn main() {
///     let e = Error::from("error message");
///     assert_eq!(e.to_string().as_str(), "App => error message");
/// }
/// ```
#[macro_export]
macro_rules! error_rules {
    () => {};

    (
        _from $arg:ty
    ) => {
        impl ::std::convert::From<$arg> for Error {
            #[inline]
            fn from(e: $arg) -> Error { Error(e.into()) }
        }
    };

    (
        from { $($arg:ty,)* }
        $($tail:tt)*
    ) => {
        $( error_rules! { _from $arg } )*
        error_rules! { $($tail)* }
    };

    (
        from { $($arg:ty),* }
        $($tail:tt)*
    ) => {
        error_rules! { from { $($arg,)* } }
        error_rules! { $($tail)* }
    };

    (
        _error_display $name:ident, ($text:expr)
    ) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, $text)
            }
        }
    };

    (
        _error_display $name:ident, ($fmt:expr, $($arg:tt),+)
    ) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, $fmt, $(self.$arg),*)
            }
        }
    };

    (
        _error_stuff $name:ident $display:tt
    ) => {
            error_rules! { _error_display $name, $display }
            impl ::std::error::Error for $name {}
            error_rules! { _from $name }
    };

    ( _error ) => {};

    (
        _error $name:ident => $display:tt
        $($tail:tt)*
    ) => {
        #[derive(Debug)]
        pub struct $name;
        error_rules! { _error_stuff $name $display }
        error_rules! { _error $($tail)* }
    };

    (
        _error $name:ident ( $($field:tt),+ ) => $display:tt
        $($tail:tt)*
    ) => {
        #[derive(Debug)]
        pub struct $name ( $(pub $field),* );
        error_rules! { _error_stuff $name $display }
        error_rules! { _error $($tail)* }
    };

    (
        _error $name:ident { $($field:tt: $type:ty,)+ } => $display:tt
        $($tail:tt)*
    ) => {
        #[derive(Debug)]
        pub struct $name { $(pub $field: $type),* }
        error_rules! { _error_stuff $name $display }
        error_rules! { _error $($tail)* }
    };

    (
        _error $name:ident { $($field:tt: $type:ty),+ } => $display:tt
        $($tail:tt)*
    ) => {
        error_rules! { _error $name { $($field:tt: $type:ty,)* } => $display }
        error_rules! { _error $($tail)* }
    };

    (
        errors { $($error:tt)* }
        $($tail:tt)*
    ) => {
        error_rules! { _error $($error)* }
        error_rules! { $($tail)* }
    };

    (
        name = $name:tt
        $($tail:tt)*
    ) => {
        pub struct Error(Box<dyn ::std::error::Error>);
        pub type Result<T> = ::std::result::Result<T, Error>;

        impl ::std::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_tuple($name)
                    .field(&self.0)
                    .finish()
            }
        }

        impl ::std::fmt::Display for Error {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{} => ", $name)?;
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::error::Error for Error {
            #[inline]
            fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                Some(self.0.as_ref())
            }
        }

        error_rules! { _from &str }
        error_rules! { _from String }

        error_rules! { $($tail)* }
    };
}


/// Exits a function and returns Error
///
/// Usage:
///
/// ```
/// #[macro_use]
/// extern crate error_rules;
///
/// error_rules! {
///     name = "App"
/// }
///
/// fn run() -> Result<()> {
///     bail!("run error");
/// }
///
/// fn main() {
///     if let Err(e) = run() {
///         println!("{}", e);
///     }
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
/// #[macro_use]
/// extern crate error_rules;
///
/// error_rules! {
///     name = "App"
/// }
///
/// fn run() -> Result<()> {
///     ensure!(false, "ensure error");
///     Ok(())
/// }
///
/// fn main() {
///     if let Err(e) = run() {
///         println!("{}", e);
///     }
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
