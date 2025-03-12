use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Debug)]
pub(crate) struct ByteSetRangeIter<'a> {
    set: &'a ByteSet,
    b: usize,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
impl<'a> Iterator for ByteSetRangeIter<'a> {
    type Item = (u8, u8);
    fn next(&mut self) -> Option<(u8, u8)> {
        let asu8 = |n: usize| u8::try_from(n).unwrap();
        while self.b <= 255 {
            let start = asu8(self.b);
            self.b += 1;
            if !self.set.contains(start) {
                continue;
            }
            let mut end = start;
            while self.b <= 255 && self.set.contains(asu8(self.b)) {
                end = asu8(self.b);
                self.b += 1;
            }
            return Some((start, end));
        }
        None
    }
}
impl ByteSet {
    pub(crate) fn empty() -> ByteSet {}
    pub(crate) fn add(&mut self, byte: u8) {}
    pub(crate) fn remove(&mut self, byte: u8) {}
    pub(crate) fn contains(&self, byte: u8) -> bool {
        let bucket = byte / 128;
        let bit = byte % 128;
        self.bits.0[usize::from(bucket)] & (1 << bit) > 0
    }
    pub(crate) fn contains_range(&self, start: u8, end: u8) -> bool {}
    pub(crate) fn iter(&self) -> ByteSetIter {}
    pub(crate) fn iter_ranges(&self) -> ByteSetRangeIter {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_empty(&self) -> bool {}
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteSet, usize), DeserializeError> {}
    pub(crate) fn write_to<E: crate::util::wire::Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
}
