use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Debug)]
pub struct ByteClassElements<'a> {
    classes: &'a ByteClasses,
    class: Unit,
    byte: usize,
}
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
impl<'a> Iterator for ByteClassElements<'a> {
    type Item = Unit;
    fn next(&mut self) -> Option<Unit> {
        while self.byte < 256 {
            let byte = u8::try_from(self.byte).unwrap();
            self.byte += 1;
            if self.class.is_byte(self.classes.get(byte)) {
                return Some(Unit::u8(byte));
            }
        }
        if self.byte < 257 {
            self.byte += 1;
            if self.class.is_eoi() {
                return Some(Unit::eoi(256));
            }
        }
        None
    }
}
impl Unit {
    pub fn u8(byte: u8) -> Unit {
        Unit(UnitKind::U8(byte))
    }
    pub fn eoi(num_byte_equiv_classes: usize) -> Unit {
        assert!(
            num_byte_equiv_classes <= 256,
            "max number of byte-based equivalent classes is 256, but got {}",
            num_byte_equiv_classes,
        );
        Unit(UnitKind::EOI(u16::try_from(num_byte_equiv_classes).unwrap()))
    }
    pub fn as_u8(self) -> Option<u8> {}
    pub fn as_eoi(self) -> Option<u16> {}
    pub fn as_usize(self) -> usize {}
    pub fn is_byte(self, byte: u8) -> bool {
        self.as_u8().map_or(false, |b| b == byte)
    }
    pub fn is_eoi(self) -> bool {
        self.as_eoi().is_some()
    }
    pub fn is_word_byte(self) -> bool {}
}
impl ByteClasses {
    #[inline]
    pub fn empty() -> ByteClasses {}
    #[inline]
    pub fn singletons() -> ByteClasses {}
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteClasses, usize), DeserializeError> {}
    pub(crate) fn write_to(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
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
