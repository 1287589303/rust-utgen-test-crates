use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Clone, Copy, Default, Eq, PartialEq)]
struct BitSet([u128; 2]);
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
impl core::fmt::Debug for BitSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut fmtd = f.debug_set();
        for b in 0u8..=255 {
            if (ByteSet { bits: *self }).contains(b) {
                fmtd.entry(&b);
            }
        }
        fmtd.finish()
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
