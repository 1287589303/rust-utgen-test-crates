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
    pub fn as_slice(&self) -> &[Utf8Range] {}
    pub fn len(&self) -> usize {}
    pub fn reverse(&mut self) {
        match *self {
            Utf8Sequence::One(_) => {}
            Utf8Sequence::Two(ref mut x) => x.reverse(),
            Utf8Sequence::Three(ref mut x) => x.reverse(),
            Utf8Sequence::Four(ref mut x) => x.reverse(),
        }
    }
    pub fn matches(&self, bytes: &[u8]) -> bool {}
}
