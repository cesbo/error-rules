# error-rules

[![Latest Version](https://img.shields.io/crates/v/error-rules.svg)](https://crates.io/crates/error-rules)
[![docs](https://docs.rs/error-rules/badge.svg)](https://docs.rs/error-rules)

Chained error handling in Rust.

## Intro

Key feature of the `error-rules` is chained error handling without pain.

For example your application have nested modules: app -> garage -> car -> engine. \
But how to know where this error happens? Should be saved error context for each module.
To do that could be use `.map_err()` before each `?` operator. But this way is too verbose.

The `error-rules` macro will do that automaticaly.
Idea is simple, each module has own error handler with configurable display text.
It pass source error wrapped into own error handler with custom display text.
So app will get error with text like: "Garage => Car => Engine => resource temporarily unavailable"
