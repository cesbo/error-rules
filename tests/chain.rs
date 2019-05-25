#[macro_use]
extern crate error_rules;


#[test]
fn test_note() {
    mod e {
        error_rules! { "test error" }
    }

    let mut e = e::Error::from("message");
    e.note(" (note)");
    assert_eq!(
        e.to_string().as_str(),
        "test error (note) => message");
}
