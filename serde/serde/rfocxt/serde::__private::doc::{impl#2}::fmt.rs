use crate::lib::*;
use crate::ser;
#[derive(Debug)]
pub struct Error;
impl Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
