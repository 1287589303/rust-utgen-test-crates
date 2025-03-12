use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
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
impl ByteClasses {
    #[inline]
    pub fn empty() -> ByteClasses {}
    #[inline]
    pub fn singletons() -> ByteClasses {}
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteClasses, usize), DeserializeError> {}
    pub(crate) fn write_to(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("byte class map"));
        }
        for b in 0..=255 {
            dst[0] = self.get(b);
            dst = &mut dst[1..];
        }
        Ok(nwrite)
    }
    pub(crate) fn write_to_len(&self) -> usize {
        256
    }
    #[inline]
    pub fn set(&mut self, byte: u8, class: u8) {}
    #[inline]
    pub fn get(&self, byte: u8) -> u8 {
        self.0[usize::from(byte)]
    }
    #[inline]
    pub fn get_by_unit(&self, unit: Unit) -> usize {}
    #[inline]
    pub fn eoi(&self) -> Unit {}
    #[inline]
    pub fn alphabet_len(&self) -> usize {}
    #[inline]
    pub fn stride2(&self) -> usize {}
    #[inline]
    pub fn is_singleton(&self) -> bool {}
    #[inline]
    pub fn iter(&self) -> ByteClassIter<'_> {}
    pub fn representatives<R: core::ops::RangeBounds<u8>>(
        &self,
        range: R,
    ) -> ByteClassRepresentatives<'_> {}
    #[inline]
    pub fn elements(&self, class: Unit) -> ByteClassElements {}
    fn element_ranges(&self, class: Unit) -> ByteClassElementRanges {}
}
impl SerializeError {
    pub(crate) fn buffer_too_small(what: &'static str) -> SerializeError {
        SerializeError { what }
    }
}
