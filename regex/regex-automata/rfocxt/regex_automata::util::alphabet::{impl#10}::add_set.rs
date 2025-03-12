use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub(crate) struct ByteClassSet(ByteSet);
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[derive(Debug)]
pub(crate) struct ByteSetRangeIter<'a> {
    set: &'a ByteSet,
    b: usize,
}
#[cfg(feature = "alloc")]
impl ByteClassSet {
    pub(crate) fn empty() -> Self {
        ByteClassSet(ByteSet::empty())
    }
    pub(crate) fn set_range(&mut self, start: u8, end: u8) {
        debug_assert!(start <= end);
        if start > 0 {
            self.0.add(start - 1);
        }
        self.0.add(end);
    }
    pub(crate) fn add_set(&mut self, set: &ByteSet) {
        for (start, end) in set.iter_ranges() {
            self.set_range(start, end);
        }
    }
    pub(crate) fn byte_classes(&self) -> ByteClasses {}
}
impl ByteSet {
    pub(crate) fn empty() -> ByteSet {}
    pub(crate) fn add(&mut self, byte: u8) {}
    pub(crate) fn remove(&mut self, byte: u8) {}
    pub(crate) fn contains(&self, byte: u8) -> bool {}
    pub(crate) fn contains_range(&self, start: u8, end: u8) -> bool {}
    pub(crate) fn iter(&self) -> ByteSetIter {}
    pub(crate) fn iter_ranges(&self) -> ByteSetRangeIter {
        ByteSetRangeIter {
            set: self,
            b: 0,
        }
    }
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
