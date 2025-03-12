#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    string::String, vec::Vec,
};
use core::{fmt, slice, str};
pub use self::ascii_set::{AsciiSet, CONTROLS, NON_ALPHANUMERIC};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PercentEncode<'a> {
    bytes: &'a [u8],
    ascii_set: &'static AsciiSet,
}
#[derive(Debug, PartialEq, Eq)]
pub struct AsciiSet {
    mask: [Chunk; ASCII_RANGE_LEN / BITS_PER_CHUNK],
}
impl<'a> Iterator for PercentEncode<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {}
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.bytes.is_empty() { (0, Some(0)) } else { (1, Some(self.bytes.len())) }
    }
}
