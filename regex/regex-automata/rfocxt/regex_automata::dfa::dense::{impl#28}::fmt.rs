#[cfg(feature = "alloc")]
pub(crate) type OwnedDFA = DFA<alloc::vec::Vec<u32>>;
#[cfg(feature = "dfa-build")]
use core::cmp;
use core::{fmt, iter, mem::size_of, slice};
#[cfg(feature = "dfa-build")]
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec, vec::Vec,
};
#[cfg(feature = "dfa-build")]
use crate::{
    dfa::{accel::Accel, determinize, minimize::Minimizer, remapper::Remapper, sparse},
    nfa::thompson, util::{look::LookMatcher, search::MatchKind},
};
use crate::{
    dfa::{
        accel::Accels, automaton::{fmt_state_indicator, Automaton, StartError},
        special::Special, start::StartKind, DEAD,
    },
    util::{
        alphabet::{self, ByteClasses, ByteSet},
        int::{Pointer, Usize},
        prefilter::Prefilter, primitives::{PatternID, StateID},
        search::Anchored, start::{self, Start, StartByteMap},
        wire::{self, DeserializeError, Endian, SerializeError},
    },
};
const LABEL: &str = "rust-regex-automata-dfa-dense";
const VERSION: u32 = 2;
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Debug)]
pub(crate) struct StateSparseTransitionIter<'a> {
    dense: StateTransitionIter<'a>,
    cur: Option<(alphabet::Unit, alphabet::Unit, StateID)>,
}
impl<'a> fmt::Debug for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (start, end, sid)) in self.sparse_transitions().enumerate() {
            let id = if f.alternate() {
                sid.as_usize()
            } else {
                sid.as_usize() >> self.stride2
            };
            if i > 0 {
                write!(f, ", ")?;
            }
            if start == end {
                write!(f, "{:?} => {:?}", start, id)?;
            } else {
                write!(f, "{:?}-{:?} => {:?}", start, end, id)?;
            }
        }
        Ok(())
    }
}
impl<'a> State<'a> {
    pub(crate) fn transitions(&self) -> StateTransitionIter<'_> {}
    pub(crate) fn sparse_transitions(&self) -> StateSparseTransitionIter<'_> {
        StateSparseTransitionIter {
            dense: self.transitions(),
            cur: None,
        }
    }
    pub(crate) fn id(&self) -> StateID {}
    #[cfg(feature = "dfa-build")]
    fn accelerate(&self, classes: &ByteClasses) -> Option<Accel> {}
}
