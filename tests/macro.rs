use error_rules::*;


#[test]
fn test_error_kind_wo_arg() {
    #[derive(Debug, Error)]
    enum E {
        #[error_kind("custom")]
        Custom,
    }

    assert_eq!(E::Custom.to_string().as_str(), "custom");
}


#[test]
fn test_error_kind_w1_arg() {
    #[derive(Debug, Error)]
    enum E {
        #[error_kind("custom:{}", 0)]
        Custom(usize),
    }

    assert_eq!(E::Custom(100).to_string().as_str(), "custom:100");
}


#[test]
fn test_error_kind_w2_arg() {
    #[derive(Debug, Error)]
    enum E {
        #[error_kind("custom:{}:{}", 0, 1)]
        Custom(usize, usize),
    }

    assert_eq!(E::Custom(100, 200).to_string().as_str(), "custom:100:200");
}


#[test]
fn test_error_from() {
    use std::io;

    #[derive(Debug, Error)]
    enum E {
        #[error_from("io:{}", 0)]
        Io(io::Error),
    }

    let e = E::Io(io::Error::from(io::ErrorKind::PermissionDenied));
    assert_eq!(e.to_string().as_str(), "io:permission denied");
    assert_eq!(std::error::Error::source(&e).unwrap().to_string().as_str(), "permission denied");
}


#[test]
fn test_error_from_wo_attrs() {
    use std::io;

    #[derive(Debug, Error)]
    enum E {
        #[error_from]
        Io(io::Error),
    }

    let e: E = io::Error::from(io::ErrorKind::PermissionDenied).into();
    assert_eq!(e.to_string().as_str(), "permission denied");
}


#[test]
fn test_error_prefix() {
    use std::io;

    #[derive(Debug, Error)]
    #[error_prefix = "App"]
    enum E {
        #[error_from]
        Io(io::Error),
    }

    let e: E = io::Error::from(io::ErrorKind::PermissionDenied).into();
    assert_eq!(e.to_string().as_str(), "App: permission denied");
}
