use crate::util::{escape::DebugByte, wire::{self, DeserializeError, SerializeError}};
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Clone, Copy)]
pub struct DebugByte(pub u8);
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
impl core::fmt::Debug for Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            UnitKind::U8(b) => write!(f, "{:?}", DebugByte(b)),
            UnitKind::EOI(_) => write!(f, "EOI"),
        }
    }
}
