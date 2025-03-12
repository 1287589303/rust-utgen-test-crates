use core::{char, fmt, iter::FusedIterator, slice};
use alloc::{vec, vec::Vec};
const MAX_UTF8_BYTES: usize = 4;
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Utf8Range {
    /// Start of byte range (inclusive).
    pub start: u8,
    /// End of byte range (inclusive).
    pub end: u8,
}
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Utf8Sequence {
    /// One byte range.
    One(Utf8Range),
    /// Two successive byte ranges.
    Two([Utf8Range; 2]),
    /// Three successive byte ranges.
    Three([Utf8Range; 3]),
    /// Four successive byte ranges.
    Four([Utf8Range; 4]),
}
impl fmt::Debug for Utf8Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Utf8Sequence::*;
        match *self {
            One(ref r) => write!(f, "{:?}", r),
            Two(ref r) => write!(f, "{:?}{:?}", r[0], r[1]),
            Three(ref r) => write!(f, "{:?}{:?}{:?}", r[0], r[1], r[2]),
            Four(ref r) => write!(f, "{:?}{:?}{:?}{:?}", r[0], r[1], r[2], r[3]),
        }
    }
}
