use core::{char, fmt, iter::FusedIterator, slice};
use alloc::{vec, vec::Vec};
const MAX_UTF8_BYTES: usize = 4;
struct ScalarRange {
    start: u32,
    end: u32,
}
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Utf8Range {
    /// Start of byte range (inclusive).
    pub start: u8,
    /// End of byte range (inclusive).
    pub end: u8,
}
impl ScalarRange {
    fn split(&self) -> Option<(ScalarRange, ScalarRange)> {}
    fn is_valid(&self) -> bool {}
    fn as_ascii(&self) -> Option<Utf8Range> {
        if self.is_ascii() {
            let start = u8::try_from(self.start).unwrap();
            let end = u8::try_from(self.end).unwrap();
            Some(Utf8Range::new(start, end))
        } else {
            None
        }
    }
    fn is_ascii(&self) -> bool {
        self.is_valid() && self.end <= 0x7f
    }
    fn encode(&self, start: &mut [u8], end: &mut [u8]) -> usize {}
}
impl Utf8Range {
    fn new(start: u8, end: u8) -> Self {
        Utf8Range { start, end }
    }
    pub fn matches(&self, b: u8) -> bool {}
}
