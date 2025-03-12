#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    string::String, vec::Vec,
};
use core::{fmt, slice, str};
pub use self::ascii_set::{AsciiSet, CONTROLS, NON_ALPHANUMERIC};
#[derive(Debug, PartialEq, Eq)]
pub struct AsciiSet {
    mask: [Chunk; ASCII_RANGE_LEN / BITS_PER_CHUNK],
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PercentEncode<'a> {
    bytes: &'a [u8],
    ascii_set: &'static AsciiSet,
}
#[inline]
pub fn utf8_percent_encode<'a>(
    input: &'a str,
    ascii_set: &'static AsciiSet,
) -> PercentEncode<'a> {
    percent_encode(input.as_bytes(), ascii_set)
}
#[inline]
pub fn percent_encode<'a>(
    input: &'a [u8],
    ascii_set: &'static AsciiSet,
) -> PercentEncode<'a> {
    PercentEncode {
        bytes: input,
        ascii_set,
    }
}
