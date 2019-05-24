#[macro_use]
mod error_rules;

use std::error::Error;


#[derive(Debug)]
/// Iterator over the error chain using the `Error::source()` method.
pub struct Iter<'a>(Option<&'a Error>);


impl<'a> Iter<'a> {
    #[inline]
    pub fn new(err: &'a Error) -> Iter<'a> { Iter(Some(err)) }
}


impl<'a> Iterator for Iter<'a> {
    type Item = &'a Error;

    #[inline]
    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        if let Some(e) = self.0.take() {
            self.0 = e.source();
            Some(e)
        } else {
            None
        }
    }
}
