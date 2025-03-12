use core::{char, fmt, iter::FusedIterator, slice};
use alloc::{vec, vec::Vec};
const MAX_UTF8_BYTES: usize = 4;
struct ScalarRange {
    start: u32,
    end: u32,
}
impl ScalarRange {
    fn split(&self) -> Option<(ScalarRange, ScalarRange)> {}
    fn is_valid(&self) -> bool {}
    fn as_ascii(&self) -> Option<Utf8Range> {}
    fn is_ascii(&self) -> bool {}
    fn encode(&self, start: &mut [u8], end: &mut [u8]) -> usize {
        let cs = char::from_u32(self.start).unwrap();
        let ce = char::from_u32(self.end).unwrap();
        let ss = cs.encode_utf8(start);
        let se = ce.encode_utf8(end);
        assert_eq!(ss.len(), se.len());
        ss.len()
    }
}
