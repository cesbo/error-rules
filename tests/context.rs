use std::fmt;

#[macro_use]
extern crate error_rules;


#[test]
fn test_context() {
    mod e {
        error_rules! { "test error" }
    }

    struct Foo(usize);
    impl fmt::Display for Foo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "foo-{}", self.0)
        }
    }

    let foo = Foo(1234);

    let r: Result<(), String> = Err("error".to_owned());
    let r = e::ResultExt::context(r, foo);

    match r {
        Ok(_) => unreachable!(),
        Err(e) => {
            assert_eq!(
                e.to_string().as_str(),
                "test error foo-1234 => error");
        }
    };
}
