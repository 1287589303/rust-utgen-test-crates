use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Debug)]
struct ByteClassElementRanges<'a> {
    elements: ByteClassElements<'a>,
    range: Option<(Unit, Unit)>,
}
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Debug)]
pub struct ByteClassElements<'a> {
    classes: &'a ByteClasses,
    class: Unit,
    byte: usize,
}
impl<'a> Iterator for ByteClassElementRanges<'a> {
    type Item = (Unit, Unit);
    fn next(&mut self) -> Option<(Unit, Unit)> {
        loop {
            let element = match self.elements.next() {
                None => return self.range.take(),
                Some(element) => element,
            };
            match self.range.take() {
                None => {
                    self.range = Some((element, element));
                }
                Some((start, end)) => {
                    if end.as_usize() + 1 != element.as_usize() || element.is_eoi() {
                        self.range = Some((element, element));
                        return Some((start, end));
                    }
                    self.range = Some((start, element));
                }
            }
        }
    }
}
impl Unit {
    pub fn u8(byte: u8) -> Unit {}
    pub fn eoi(num_byte_equiv_classes: usize) -> Unit {}
    pub fn as_u8(self) -> Option<u8> {}
    pub fn as_eoi(self) -> Option<u16> {}
    pub fn as_usize(self) -> usize {
        match self.0 {
            UnitKind::U8(b) => usize::from(b),
            UnitKind::EOI(eoi) => usize::from(eoi),
        }
    }
    pub fn is_byte(self, byte: u8) -> bool {}
    pub fn is_eoi(self) -> bool {
        self.as_eoi().is_some()
    }
    pub fn is_word_byte(self) -> bool {}
}
