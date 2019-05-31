use std::io;
use error_rules::*;


pub struct Human;


#[derive(Debug, Error)]
pub enum HumanError {
    #[error_from("Human IO: {}", 0)]
    Io(io::Error),
}


impl Human {
    fn io_error(&self) -> std::result::Result<(), io::Error> {
        Err(io::Error::from(io::ErrorKind::PermissionDenied))
    }

    pub fn invoke_failure(&self) -> Result<(), HumanError> {
        self.io_error()?;
        unreachable!()
    }
}


#[derive(Debug, Error)]
pub enum BikeError {
    #[error_from("Bike IO: {}", 0)]
    Io(io::Error),

    #[error_from("Bike: {}", 0)]
    Human(HumanError),
}


pub struct Bike {
    human: Human,
}


impl Bike {
    pub fn ride(&self) -> Result<(), BikeError> {
        self.human.invoke_failure()?;
        unreachable!()
    }

    fn io_error(&self) -> std::result::Result<(), io::Error> {
        Err(io::Error::from(io::ErrorKind::PermissionDenied))
    }

    pub fn invoke_failure(&self) -> Result<(), BikeError> {
        self.io_error()?;
        unreachable!()
    }
}


#[test]
fn test_bike() {
    let h = Human;
    let b = Bike { human: h };

    let error = b.ride().unwrap_err();
    assert_eq!(error.to_string().as_str(),
        "Bike: Human IO: permission denied");

    let error = b.invoke_failure().unwrap_err();
    assert_eq!(error.to_string().as_str(),
        "Bike IO: permission denied");
}
