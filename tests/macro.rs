#[macro_use]
extern crate error_rules;

mod e {
    error_rules! {
        name = "Test"
        errors {
            CustomError => ("custom error")
        }
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
