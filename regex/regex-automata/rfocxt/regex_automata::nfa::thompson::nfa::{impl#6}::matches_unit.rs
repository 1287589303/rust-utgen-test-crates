use core::{fmt, mem};
use alloc::{boxed::Box, format, string::String, sync::Arc, vec, vec::Vec};
#[cfg(feature = "syntax")]
use crate::nfa::thompson::{
    compiler::{Compiler, Config},
    error::BuildError,
};
use crate::{
    nfa::thompson::builder::Builder,
    util::{
        alphabet::{self, ByteClassSet, ByteClasses},
        captures::{GroupInfo, GroupInfoError},
        look::{Look, LookMatcher, LookSet},
        primitives::{IteratorIndexExt, PatternID, PatternIDIter, SmallIndex, StateID},
        sparse_set::SparseSet,
    },
};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseTransitions {
    /// The sorted sequence of non-overlapping transitions.
    pub transitions: Box<[Transition]>,
}
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
}
#[derive(Clone)]
struct Transition {
    /// The byte range.
    range: Utf8Range,
    /// The next state to transition to.
    next_id: StateID,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl SparseTransitions {
    #[inline]
    pub fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID> {}
    #[inline]
    pub(crate) fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID> {
        unit.as_u8().map_or(None, |byte| self.matches_byte(byte))
    }
    #[inline]
    pub fn matches_byte(&self, byte: u8) -> Option<StateID> {}
}
impl Unit {
    pub fn u8(byte: u8) -> Unit {}
    pub fn eoi(num_byte_equiv_classes: usize) -> Unit {}
    pub fn as_u8(self) -> Option<u8> {
        match self.0 {
            UnitKind::U8(b) => Some(b),
            UnitKind::EOI(_) => None,
        }
    }
    pub fn as_eoi(self) -> Option<u16> {}
    pub fn as_usize(self) -> usize {}
    pub fn is_byte(self, byte: u8) -> bool {}
    pub fn is_eoi(self) -> bool {}
    pub fn is_word_byte(self) -> bool {}
}
