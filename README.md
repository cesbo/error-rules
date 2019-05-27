# error-rules

[![Latest Version](https://img.shields.io/crates/v/error-rules.svg)](https://crates.io/crates/error-rules)
[![docs](https://docs.rs/error-rules/badge.svg)](https://docs.rs/error-rules)

Chained error handling in Rust.

## Intro

Key feature of the `error-rules` is chained error handling.

Idea is simple, each module has own error handler.
Source error wrapped into error handler with configurable display text.

## Usage

```rust
#[macro_use]
extern crate error_rules;

pub mod human {
    use std::io;

    error_rules! {
        Error => ("Human => {}", error),
        io::Error,
    }

    #[derive(Default)]
    pub struct Human;

    impl Human {
        pub fn invoke_failure(&self) -> Result<()> {
            bail!(io::Error::from(io::ErrorKind::PermissionDenied));
        }
    }
}

pub mod bike {
    use std::io;
    use crate::human;

    error_rules! {
        Error => ("Bike => {}", error),
        io::Error,
        human::Error,
    }

    #[derive(Default)]
    pub struct Bike(human::Human);

    impl Bike {
        pub fn ride(&self) -> Result<()> {
            self.0.invoke_failure()?;
            Ok(())
        }

        pub fn invoke_failure(&self) -> Result<()> {
            bail!(io::Error::from(io::ErrorKind::PermissionDenied));
        }
    }
}

let b = bike::Bike::default();

let error = b.ride().unwrap_err();
assert_eq!(
    error.to_string().as_str(),
    "Bike => Human => permission denied"
);

let error = b.invoke_failure().unwrap_err();
assert_eq!(
    error.to_string().as_str(),
    "Bike => permission denied"
);
```
