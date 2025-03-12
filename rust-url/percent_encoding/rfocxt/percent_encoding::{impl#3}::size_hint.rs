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
impl Iterator for PercentDecode<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {}
    fn size_hint(&self) -> (usize, Option<usize>) {
        let bytes = self.bytes.len();
        ((bytes + 2) / 3, Some(bytes))
    }
}
