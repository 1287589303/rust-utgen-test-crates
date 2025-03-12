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
impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = formatter.debug_tuple("Error");
        #[cfg(any(feature = "std", feature = "alloc"))] debug.field(&self.err);
        debug.finish()
    }
}
