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
pub struct DenseTransitions {
    /// A dense representation of this state's transitions on the heap. This
    /// always has length 256.
    pub transitions: Box<[StateID]>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl DenseTransitions {
    #[inline]
    pub fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID> {
        haystack.get(at).and_then(|&b| self.matches_byte(b))
    }
    #[inline]
    pub(crate) fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID> {}
    #[inline]
    pub fn matches_byte(&self, byte: u8) -> Option<StateID> {}
    pub(crate) fn iter(&self) -> impl Iterator<Item = Transition> + '_ {}
}
