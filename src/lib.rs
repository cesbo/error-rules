/// Macro for generating error types
///
/// Implements `Error`, `Result` types
///
/// Parts of the `error_rules!` macro:
///
/// - `name` - defining module name for error chaining. Will be a part of resulted string
/// - `from` - defining types for conversion into `Error`
/// - `errors` - defining custom error types
///
/// ```
/// #[macro_use]
/// extern crate error_rules;
///
/// error_rules! {
///     name = "App"
///     from { &str }
///     errors {
///         CustomError => ("custom error")
///     }
/// }
///
/// fn main() {
///     let e = Error("error message".into());
///     println!("{}", e);
/// }
/// ```
#[macro_export]
macro_rules! error_rules {
    () => {};

    (
        from { $($arg:ty)* }
        $($tail:tt)*
    ) => {
        $( impl ::std::convert::From<$arg> for Error {
            #[inline]
            fn from(e: $arg) -> Error { Error(e.into()) }
        } )*

        error_rules! { $($tail)* }
    };

    (
        _error_display ($text:expr)
    ) => {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, $text)
        }
    };

    (
        _error_display ($fmt:expr, $($arg:tt),+)
    ) => {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, $fmt, $(self.$arg),*)
        }
    };

    (
        _error_stuff $name:ident $display:tt
    ) => {
            impl ::std::fmt::Display for $name {
                error_rules! { _error_display $display }
            }

            impl ::std::error::Error for $name {}

            impl ::std::convert::From<$name> for Error {
                #[inline]
                fn from(e: $name) -> Error { Error(e.into()) }
            }
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
        pub struct $name { $(pub $field: $type),+ }
        error_rules! { _error_stuff $name $display }
        error_rules! { _error $($tail)* }
    };

    (
        _error $name:ident { $($field:tt: $type:ty),+ } => $display:tt
        $($tail:tt)*
    ) => {
        error_rules! { _error $name { $($field:tt: $type:ty,)* } => $display $($tail)* }
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
///     from { &str }
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
        return Err($e.into());
    };

    ( $fmt:expr, $($arg:tt)+ ) => {
        return Err(format!($fmt, $($arg)+).into());
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
///     from { &str }
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
    ( $cond:expr, $($e:tt),* ) => {
        if !($cond) { bail!($($e),*); }
    };
}
