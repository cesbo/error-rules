#[macro_use]
extern crate error_rules;

mod e {
    error_rules! {
        Error => ("Test => {}", error),
        CustomError => ("custom error"),
    }
}

#[test]
fn test_bail() {
    fn foo() -> e::Result<()> {
        bail!("error message");
    }

    if let Err(e) = foo() {
        assert_eq!(e.to_string().as_str(), "Test => error message");
    } else {
        unreachable!()
    }
}


#[test]
fn test_bail_args() {
    fn foo() -> e::Result<()> {
        bail!("a1:{} a2:{}", 1234, "hello");
    }

    if let Err(e) = foo() {
        assert_eq!(e.to_string().as_str(), "Test => a1:1234 a2:hello");
    } else {
        unreachable!()
    }
}


#[test]
fn test_bail_errors() {
    fn foo() -> e::Result<()> {
        bail!(e::CustomError);
    }

    if let Err(e) = foo() {
        assert_eq!(e.to_string().as_str(), "Test => custom error");
    } else {
        unreachable!()
    }
}


#[test]
fn test_ensure() {
    fn foo() -> e::Result<()> {
        ensure!(false, "error message");
        Ok(())
    }

    if let Err(e) = foo() {
        assert_eq!(e.to_string().as_str(), "Test => error message");
    } else {
        unreachable!()
    }
}


#[test]
fn test_ensure_args() {
    fn foo() -> e::Result<()> {
        ensure!(false, "a1:{} a2:{}", 1234, "hello");
        Ok(())
    }

    if let Err(e) = foo() {
        assert_eq!(e.to_string().as_str(), "Test => a1:1234 a2:hello");
    } else {
        unreachable!()
    }
}


#[test]
fn test_ensure_errors() {
    fn foo() -> e::Result<()> {
        ensure!(false, e::CustomError);
        Ok(())
    }

    if let Err(e) = foo() {
        assert_eq!(e.to_string().as_str(), "Test => custom error");
    } else {
        unreachable!()
    }
}


#[test]
fn test_custom_wo_arg() {
    mod c0 {
        error_rules! {
            Error => ("test"),
            Custom => ("custom"),
        }
    }

    assert_eq!(c0::Custom.to_string().as_str(), "custom");

    mod t0 {
        error_rules! {
            Error => ("test"),
            Custom() => ("custom"),
        }
    }

    assert_eq!(t0::Custom().to_string().as_str(), "custom");

    mod s0 {
        error_rules! {
            Error => ("test"),
            Custom{} => ("custom"),
        }
    }

    assert_eq!(s0::Custom{}.to_string().as_str(), "custom");
}

#[test]
fn test_custom_w1_arg() {
    mod t1 {
        error_rules! {
            Error => ("test"),
            Custom(usize) => ("custom:{}", 0),
        }
    }

    mod t1c {
        error_rules! {
            Error => ("test"),
            Custom(usize,) => ("custom:{}", 0),
        }
    }

    assert_eq!(t1::Custom(100).to_string().as_str(), "custom:100");
    assert_eq!(t1c::Custom(100).to_string().as_str(), "custom:100");

    mod s1 {
        error_rules! {
            Error => ("test"),
            Custom{ v1: usize } => ("custom:{}", v1),
        }
    }

    mod s1c {
        error_rules! {
            Error => ("test"),
            Custom {
                v1: usize,
            } => ("custom:{}", v1),
        }
    }

    assert_eq!(s1::Custom{ v1: 100 }.to_string().as_str(), "custom:100");
    assert_eq!(s1c::Custom{ v1: 100 }.to_string().as_str(), "custom:100");
}

#[test]
fn test_custom_w2_arg() {
    mod t2 {
        error_rules! {
            Error => ("test"),
            Custom(usize, usize) => ("custom:{}:{}", 0, 1),
        }
    }

    assert_eq!(t2::Custom(100, 200).to_string().as_str(), "custom:100:200");

    mod s2 {
        error_rules! {
            Error => ("test"),
            Custom {
                v1: usize,
                v2: usize,
            } => ("custom:{}:{}", v1, v2),
        }
    }

    assert_eq!(s2::Custom{ v1: 100, v2: 200 }.to_string().as_str(), "custom:100:200");
}
