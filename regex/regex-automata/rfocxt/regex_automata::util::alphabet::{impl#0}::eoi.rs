use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum UnitKind {
    /// Represents a byte value, or more typically, an equivalence class
    /// represented as a byte value.
    U8(u8),
    /// Represents the "end of input" sentinel. We regretably use a `u16`
    /// here since the maximum sentinel value is `256`. Thankfully, we don't
    /// actually store a `Unit` anywhere, so this extra space shouldn't be too
    /// bad.
    EOI(u16),
}
impl Unit {
    pub fn u8(byte: u8) -> Unit {}
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
    pub fn is_byte(self, byte: u8) -> bool {}
    pub fn is_eoi(self) -> bool {}
    pub fn is_word_byte(self) -> bool {}
}
