use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Debug)]
pub struct ByteClassRepresentatives<'a> {
    classes: &'a ByteClasses,
    cur_byte: usize,
    end_byte: Option<usize>,
    last_class: Option<u8>,
}
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
impl<'a> Iterator for ByteClassRepresentatives<'a> {
    type Item = Unit;
    fn next(&mut self) -> Option<Unit> {
        while self.cur_byte < self.end_byte.unwrap_or(256) {
            let byte = u8::try_from(self.cur_byte).unwrap();
            let class = self.classes.get(byte);
            self.cur_byte += 1;
            if self.last_class != Some(class) {
                self.last_class = Some(class);
                return Some(Unit::u8(byte));
            }
        }
        if self.cur_byte != usize::MAX && self.end_byte.is_none() {
            self.cur_byte = usize::MAX;
            return Some(self.classes.eoi());
        }
        None
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
    pub fn get(&self, byte: u8) -> u8 {
        self.0[usize::from(byte)]
    }
    #[inline]
    pub fn get_by_unit(&self, unit: Unit) -> usize {}
    #[inline]
    pub fn eoi(&self) -> Unit {
        Unit::eoi(self.alphabet_len().checked_sub(1).unwrap())
    }
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
