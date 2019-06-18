#[test]
fn test_1() {
    use error_rules::*;

    #[derive(Debug, Error)]
    enum AppError {
        #[error_from("App IO: {}", 0)]
        Io(std::io::Error),
    }

    type Result<T> = std::result::Result<T, AppError>;

    fn example() -> Result<()> {
        let _file = std::fs::File::open("not-found.txt")?;
        unreachable!()
    }

    let error = example().unwrap_err();
    assert_eq!(error.to_string().as_str(),
        "App IO: No such file or directory (os error 2)");
}


#[test]
fn test_2() {
    use error_rules::*;

    #[derive(Debug, Error)]
    enum AppError {
        #[error_kind("App: error without arguments")]
        E1,
        #[error_kind("App: code:{} message:{}", 0, 1)]
        E2(usize, String),
    }

    type Result<T> = std::result::Result<T, AppError>;

    fn example_1() -> Result<()> {
        Err(AppError::E1)
    }

    fn example_2() -> Result<()> {
        Err(AppError::E2(404, "Not Found".to_owned()))
    }

    let error = example_1().unwrap_err();
    assert_eq!(error.to_string().as_str(),
        "App: error without arguments");

    let error = example_2().unwrap_err();
    assert_eq!(error.to_string().as_str(),
        "App: code:404 message:Not Found");
}


#[test]
fn test_3() {
    use error_rules::*;

    #[derive(Debug, Error)]
    enum ModError {
        #[error_from("Mod IO: {}", 0)]
        Io(std::io::Error),
    }

    fn mod_example() -> Result<(), ModError> {
        let _file = std::fs::File::open("not-found.txt")?;
        unreachable!()
    }

    #[derive(Debug, Error)]
    enum AppError {
        #[error_from("App: {}", 0)]
        Mod(ModError),
    }

    fn app_example() -> Result<(), AppError> {
        mod_example()?;
        unreachable!()
    }

    let error = app_example().unwrap_err();
    assert_eq!(error.to_string().as_str(),
        "App: Mod IO: No such file or directory (os error 2)");
}
