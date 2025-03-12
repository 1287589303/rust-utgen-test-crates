use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Debug)]
pub struct ByteClassRepresentatives<'a> {
    classes: &'a ByteClasses,
    cur_byte: usize,
    end_byte: Option<usize>,
    last_class: Option<u8>,
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
    ) -> ByteClassRepresentatives<'_> {
        use core::ops::Bound;
        let cur_byte = match range.start_bound() {
            Bound::Included(&i) => usize::from(i),
            Bound::Excluded(&i) => usize::from(i).checked_add(1).unwrap(),
            Bound::Unbounded => 0,
        };
        let end_byte = match range.end_bound() {
            Bound::Included(&i) => Some(usize::from(i).checked_add(1).unwrap()),
            Bound::Excluded(&i) => Some(usize::from(i)),
            Bound::Unbounded => None,
        };
        assert_ne!(cur_byte, usize::MAX, "start range must be less than usize::MAX",);
        ByteClassRepresentatives {
            classes: self,
            cur_byte,
            end_byte,
            last_class: None,
        }
    }
    #[inline]
    pub fn elements(&self, class: Unit) -> ByteClassElements {}
    fn element_ranges(&self, class: Unit) -> ByteClassElementRanges {}
}
