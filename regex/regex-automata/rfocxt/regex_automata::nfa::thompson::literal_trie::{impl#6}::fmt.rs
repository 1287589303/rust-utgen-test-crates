use core::mem;
use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::{self, compiler::ThompsonRef, BuildError, Builder},
    util::primitives::{IteratorIndexExt, StateID},
};
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
}
#[derive(Clone, Copy)]
pub struct DebugByte(pub u8);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl core::fmt::Debug for Transition {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{:?} => {}", crate ::util::escape::DebugByte(self.byte), self.next
            .as_usize()
        )
    }
}
