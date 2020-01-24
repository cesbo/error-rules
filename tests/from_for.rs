use std::io::{
    self,
    Read
};

use error_rules::*;


#[derive(Debug, Error)]
#[error_prefix = "Test Prefix"]
pub enum TestSError {
    #[error_from]
    IO(io::Error),
    #[error_kind("Test kind")]
    TestKind,
}


pub type Result<T> = std::result::Result<T, TestSError>;


#[derive(Default)]
struct TestS {}


impl TestS {
    fn test_io_error(&mut self) -> Result<()> {
        let mut buf = [0; 1];
        self.read(&mut buf)?;
        Ok(())
    }

    fn test_kind_error(&self) -> Result<()> {
        Err(TestSError::TestKind)
    }
}


impl Read for TestS {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        self.test_kind_error()?;
        Ok(0)
    }
}


#[test]
fn for_from_test() {
    let mut tests = TestS::default();
    assert!(tests.test_kind_error().is_err());
    assert!(tests.test_io_error().is_err());
}