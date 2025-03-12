#[cfg(any(feature = "std", feature = "alloc"))]
type ErrorImpl = Box<str>;
#[cfg(not(any(feature = "std", feature = "alloc")))]
type ErrorImpl = ();
use crate::lib::*;
use self::private::{First, Second};
use crate::de::{
    self, size_hint, Deserializer, Expected, IntoDeserializer, SeqAccess, Visitor,
};
use crate::ser;
#[derive(Clone, PartialEq)]
pub struct Error {
    err: ErrorImpl,
}
pub struct T;
impl ser::Error for Error {
    #[cold]
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        de::Error::custom(msg)
    }
}
