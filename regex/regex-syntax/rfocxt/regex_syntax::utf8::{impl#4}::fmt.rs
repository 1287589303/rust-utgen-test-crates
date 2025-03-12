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
impl fmt::Debug for Utf8Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            write!(f, "[{:X}]", self.start)
        } else {
            write!(f, "[{:X}-{:X}]", self.start, self.end)
        }
    }
}
