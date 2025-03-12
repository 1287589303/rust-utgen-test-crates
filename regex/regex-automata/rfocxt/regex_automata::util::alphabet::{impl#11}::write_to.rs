use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Debug)]
pub struct SerializeError {
    /// The name of the thing that a buffer is too small for.
    ///
    /// Currently, the only kind of serialization error is one that is
    /// committed by a caller: providing a destination buffer that is too
    /// small to fit the serialized object. This makes sense conceptually,
    /// since every valid inhabitant of a type should be serializable.
    ///
    /// This is somewhat exposed in the public API of this crate. For example,
    /// the `to_bytes_{big,little}_endian` APIs return a `Vec<u8>` and are
    /// guaranteed to never panic or error. This is only possible because the
    /// implementation guarantees that it will allocate a `Vec<u8>` that is
    /// big enough.
    ///
    /// In summary, if a new serialization error kind needs to be added, then
    /// it will need careful consideration.
    what: &'static str,
}
#[derive(Clone, Copy, Default, Eq, PartialEq)]
struct BitSet([u128; 2]);
impl ByteSet {
    pub(crate) fn empty() -> ByteSet {}
    pub(crate) fn add(&mut self, byte: u8) {}
    pub(crate) fn remove(&mut self, byte: u8) {}
    pub(crate) fn contains(&self, byte: u8) -> bool {}
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
    ) -> Result<usize, SerializeError> {
        use core::mem::size_of;
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("byte set"));
        }
        let mut nw = 0;
        E::write_u128(self.bits.0[0], &mut dst[nw..]);
        nw += size_of::<u128>();
        E::write_u128(self.bits.0[1], &mut dst[nw..]);
        nw += size_of::<u128>();
        assert_eq!(nwrite, nw, "expected to write certain number of bytes",);
        assert_eq!(nw % 8, 0, "expected to write multiple of 8 bytes for byte set",);
        Ok(nw)
    }
    pub(crate) fn write_to_len(&self) -> usize {
        2 * core::mem::size_of::<u128>()
    }
}
impl SerializeError {
    pub(crate) fn buffer_too_small(what: &'static str) -> SerializeError {
        SerializeError { what }
    }
}
