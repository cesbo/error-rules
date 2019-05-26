#[macro_use]
mod error_rules;


/// Trait for any object that returns `Error` to set error context
pub trait ErrorContext {
    fn context<F: ::std::fmt::Write>(&self, f: &mut F);
}
