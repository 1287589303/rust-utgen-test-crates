use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub(crate) struct ByteClassSet(ByteSet);
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[cfg(feature = "alloc")]
impl Default for ByteClassSet {
    fn default() -> ByteClassSet {
        ByteClassSet::empty()
    }
}
#[cfg(feature = "alloc")]
impl ByteClassSet {
    pub(crate) fn empty() -> Self {
        ByteClassSet(ByteSet::empty())
    }
    pub(crate) fn set_range(&mut self, start: u8, end: u8) {}
    pub(crate) fn add_set(&mut self, set: &ByteSet) {}
    pub(crate) fn byte_classes(&self) -> ByteClasses {}
}
