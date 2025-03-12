use core::{char, fmt, iter::FusedIterator, slice};
use alloc::{vec, vec::Vec};
const MAX_UTF8_BYTES: usize = 4;
struct ScalarRange {
    start: u32,
    end: u32,
}
impl ScalarRange {
    fn split(&self) -> Option<(ScalarRange, ScalarRange)> {}
    fn is_valid(&self) -> bool {
        self.start <= self.end
    }
    fn as_ascii(&self) -> Option<Utf8Range> {}
    fn is_ascii(&self) -> bool {
        self.is_valid() && self.end <= 0x7f
    }
    fn encode(&self, start: &mut [u8], end: &mut [u8]) -> usize {}
}
