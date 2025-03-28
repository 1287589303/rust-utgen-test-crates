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
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
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
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
}
impl SparseTransitions {
    #[inline]
    pub fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID> {}
    #[inline]
    pub(crate) fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID> {}
    #[inline]
    pub fn matches_byte(&self, byte: u8) -> Option<StateID> {
        for t in self.transitions.iter() {
            if t.start > byte {
                break;
            } else if t.matches_byte(byte) {
                return Some(t.next);
            }
        }
        None
    }
}
impl Transition {
    pub fn matches(&self, haystack: &[u8], at: usize) -> bool {}
    pub fn matches_unit(&self, unit: alphabet::Unit) -> bool {}
    pub fn matches_byte(&self, byte: u8) -> bool {
        self.start <= byte && byte <= self.end
    }
}
