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
#[inline]
pub fn percent_decode(input: &[u8]) -> PercentDecode<'_> {
    PercentDecode {
        bytes: input.iter(),
    }
}
