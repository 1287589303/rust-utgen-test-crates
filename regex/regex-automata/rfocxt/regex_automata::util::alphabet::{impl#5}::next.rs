use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Debug)]
pub struct ByteClassIter<'a> {
    classes: &'a ByteClasses,
    i: usize,
}
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
impl<'a> Iterator for ByteClassIter<'a> {
    type Item = Unit;
    fn next(&mut self) -> Option<Unit> {
        if self.i + 1 == self.classes.alphabet_len() {
            self.i += 1;
            Some(self.classes.eoi())
        } else if self.i < self.classes.alphabet_len() {
            let class = u8::try_from(self.i).unwrap();
            self.i += 1;
            Some(Unit::u8(class))
        } else {
            None
        }
    }
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
    pub fn get(&self, byte: u8) -> u8 {}
    #[inline]
    pub fn get_by_unit(&self, unit: Unit) -> usize {}
    #[inline]
    pub fn eoi(&self) -> Unit {
        Unit::eoi(self.alphabet_len().checked_sub(1).unwrap())
    }
    #[inline]
    pub fn alphabet_len(&self) -> usize {
        usize::from(self.0[255]) + 1 + 1
    }
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
impl Unit {
    pub fn u8(byte: u8) -> Unit {
        Unit(UnitKind::U8(byte))
    }
    pub fn eoi(num_byte_equiv_classes: usize) -> Unit {}
    pub fn as_u8(self) -> Option<u8> {}
    pub fn as_eoi(self) -> Option<u16> {}
    pub fn as_usize(self) -> usize {}
    pub fn is_byte(self, byte: u8) -> bool {}
    pub fn is_eoi(self) -> bool {}
    pub fn is_word_byte(self) -> bool {}
}
