use core::{char, fmt, iter::FusedIterator, slice};
use alloc::{vec, vec::Vec};
const MAX_UTF8_BYTES: usize = 4;
pub trait Bound: Copy + Clone + Debug + Eq + PartialEq + PartialOrd + Ord {
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn as_u32(self) -> u32;
    fn increment(self) -> Self;
    fn decrement(self) -> Self;
}
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
impl<'a> IntoIterator for &'a Utf8Sequence {
    type IntoIter = slice::Iter<'a, Utf8Range>;
    type Item = &'a Utf8Range;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}
impl Utf8Sequence {
    fn from_encoded_range(start: &[u8], end: &[u8]) -> Self {
        assert_eq!(start.len(), end.len());
        match start.len() {
            2 => {
                Utf8Sequence::Two([
                    Utf8Range::new(start[0], end[0]),
                    Utf8Range::new(start[1], end[1]),
                ])
            }
            3 => {
                Utf8Sequence::Three([
                    Utf8Range::new(start[0], end[0]),
                    Utf8Range::new(start[1], end[1]),
                    Utf8Range::new(start[2], end[2]),
                ])
            }
            4 => {
                Utf8Sequence::Four([
                    Utf8Range::new(start[0], end[0]),
                    Utf8Range::new(start[1], end[1]),
                    Utf8Range::new(start[2], end[2]),
                    Utf8Range::new(start[3], end[3]),
                ])
            }
            n => unreachable!("invalid encoded length: {}", n),
        }
    }
    pub fn as_slice(&self) -> &[Utf8Range] {
        use self::Utf8Sequence::*;
        match *self {
            One(ref r) => slice::from_ref(r),
            Two(ref r) => &r[..],
            Three(ref r) => &r[..],
            Four(ref r) => &r[..],
        }
    }
    pub fn len(&self) -> usize {}
    pub fn reverse(&mut self) {}
    pub fn matches(&self, bytes: &[u8]) -> bool {}
}
