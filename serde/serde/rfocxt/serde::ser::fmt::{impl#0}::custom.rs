use crate::lib::*;
use crate::ser::{Error, Impossible, Serialize, Serializer};
pub struct T;
impl Error for fmt::Error {
    fn custom<T: Display>(_msg: T) -> Self {
        fmt::Error
    }
}
