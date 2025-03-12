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
