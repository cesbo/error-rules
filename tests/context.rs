#[macro_use]
extern crate error_rules;
use error_rules::ErrorContext;


#[test]
fn test_context() {
    mod e {
        error_rules! { "test error" }
    }

    struct Foo(usize);
    impl ErrorContext for Foo {
        fn context<F: std::fmt::Write>(&self, f: &mut F) {
            write!(f, " (foo-{})", self.0).unwrap()
        }
    }

    let foo = Foo(1234);

    let r: Result<(), String> = Err("error".to_owned());
    let r = e::ResultExt::context(r, &foo);

    match r {
        Ok(_) => unreachable!(),
        Err(e) => {
            assert_eq!(
                e.to_string().as_str(),
                "test error (foo-1234) => error");
        }
    };
}
