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
#[cfg(feature = "alloc")]
impl<'a> From<PercentDecode<'a>> for Cow<'a, [u8]> {
    fn from(iter: PercentDecode<'a>) -> Self {
        match iter.if_any() {
            Some(vec) => Cow::Owned(vec),
            None => Cow::Borrowed(iter.bytes.as_slice()),
        }
    }
}
impl<'a> PercentDecode<'a> {
    #[cfg(feature = "alloc")]
    fn if_any(&self) -> Option<Vec<u8>> {
        let mut bytes_iter = self.bytes.clone();
        while bytes_iter.any(|&b| b == b'%') {
            if let Some(decoded_byte) = after_percent_sign(&mut bytes_iter) {
                let initial_bytes = self.bytes.as_slice();
                let unchanged_bytes_len = initial_bytes.len() - bytes_iter.len() - 3;
                let mut decoded = initial_bytes[..unchanged_bytes_len].to_owned();
                decoded.push(decoded_byte);
                decoded.extend(PercentDecode { bytes: bytes_iter });
                return Some(decoded);
            }
        }
        None
    }
    #[cfg(feature = "alloc")]
    pub fn decode_utf8(self) -> Result<Cow<'a, str>, str::Utf8Error> {}
    #[cfg(feature = "alloc")]
    pub fn decode_utf8_lossy(self) -> Cow<'a, str> {}
}
