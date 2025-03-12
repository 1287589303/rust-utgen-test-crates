use core::{char, fmt, iter::FusedIterator, slice};
use alloc::{vec, vec::Vec};
const MAX_UTF8_BYTES: usize = 4;
struct ScalarRange {
    start: u32,
    end: u32,
}
impl fmt::Debug for ScalarRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ScalarRange({:X}, {:X})", self.start, self.end)
    }
}
