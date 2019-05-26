#[macro_use]
mod error_rules;

use std::{
    fmt,
    error,
    result,
};


/// Trait for any object that returns `Error` to set error context
pub trait ErrorContext {
    fn context<F: fmt::Write>(&self, f: &mut F);
}
