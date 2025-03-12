use crate::lib::*;
use crate::ser;
#[derive(Debug)]
pub struct Error;
pub struct T;
impl ser::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: Display,
    {
        unimplemented!()
    }
}
