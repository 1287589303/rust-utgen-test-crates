#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    string::String, vec::Vec,
};
use core::{fmt, slice, str};
pub use self::ascii_set::{AsciiSet, CONTROLS, NON_ALPHANUMERIC};
#[derive(Clone, Debug)]
pub struct PercentDecode<'a> {
    bytes: slice::Iter<'a, u8>,
}
impl<'a> PercentDecode<'a> {
    #[cfg(feature = "alloc")]
    fn if_any(&self) -> Option<Vec<u8>> {}
    #[cfg(feature = "alloc")]
    pub fn decode_utf8(self) -> Result<Cow<'a, str>, str::Utf8Error> {
        match self.clone().into() {
            Cow::Borrowed(bytes) => {
                match str::from_utf8(bytes) {
                    Ok(s) => Ok(s.into()),
                    Err(e) => Err(e),
                }
            }
            Cow::Owned(bytes) => {
                match String::from_utf8(bytes) {
                    Ok(s) => Ok(s.into()),
                    Err(e) => Err(e.utf8_error()),
                }
            }
        }
    }
    #[cfg(feature = "alloc")]
    pub fn decode_utf8_lossy(self) -> Cow<'a, str> {}
}
