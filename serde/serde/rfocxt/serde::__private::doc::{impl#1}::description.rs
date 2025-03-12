use crate::lib::*;
use crate::ser;
#[derive(Debug)]
pub struct Error;
#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        unimplemented!()
    }
}
