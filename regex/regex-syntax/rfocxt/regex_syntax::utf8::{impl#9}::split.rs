use core::{char, fmt, iter::FusedIterator, slice};
use alloc::{vec, vec::Vec};
const MAX_UTF8_BYTES: usize = 4;
struct ScalarRange {
    start: u32,
    end: u32,
}
impl ScalarRange {
    fn split(&self) -> Option<(ScalarRange, ScalarRange)> {
        if self.start < 0xE000 && self.end > 0xD7FF {
            Some((
                ScalarRange {
                    start: self.start,
                    end: 0xD7FF,
                },
                ScalarRange {
                    start: 0xE000,
                    end: self.end,
                },
            ))
        } else {
            None
        }
    }
    fn is_valid(&self) -> bool {}
    fn as_ascii(&self) -> Option<Utf8Range> {}
    fn is_ascii(&self) -> bool {}
    fn encode(&self, start: &mut [u8], end: &mut [u8]) -> usize {}
}
