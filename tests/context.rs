#[macro_use]
extern crate error_rules;

use std::fmt;

use error_rules::ErrorContext;


#[test]
fn test_context() {
    mod e {
        error_rules! { "test error" }
    }

    struct Foo(usize);
    impl ErrorContext for Foo {
        fn context<F: fmt::Write>(&self, f: &mut F) {
            write!(f, " (foo-{})", self.0).unwrap()
        }
    }

    let foo = Foo(1234);

    let mut e = e::Error::from("message");
    e.context(&foo);

    assert_eq!(
        e.to_string().as_str(),
        "test error (note) => message");
}
