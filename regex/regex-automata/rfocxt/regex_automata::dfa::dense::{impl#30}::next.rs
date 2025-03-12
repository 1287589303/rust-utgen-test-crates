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
#[derive(Debug)]
pub(crate) struct StateSparseTransitionIter<'a> {
    dense: StateTransitionIter<'a>,
    cur: Option<(alphabet::Unit, alphabet::Unit, StateID)>,
}
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Debug)]
pub(crate) struct StateTransitionIter<'a> {
    len: usize,
    it: iter::Enumerate<slice::Iter<'a, StateID>>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl<'a> Iterator for StateSparseTransitionIter<'a> {
    type Item = (alphabet::Unit, alphabet::Unit, StateID);
    fn next(&mut self) -> Option<(alphabet::Unit, alphabet::Unit, StateID)> {
        while let Some((unit, next)) = self.dense.next() {
            let (prev_start, prev_end, prev_next) = match self.cur {
                Some(t) => t,
                None => {
                    self.cur = Some((unit, unit, next));
                    continue;
                }
            };
            if prev_next == next && !unit.is_eoi() {
                self.cur = Some((prev_start, unit, prev_next));
            } else {
                self.cur = Some((unit, unit, next));
                if prev_next != DEAD {
                    return Some((prev_start, prev_end, prev_next));
                }
            }
        }
        if let Some((start, end, next)) = self.cur.take() {
            if next != DEAD {
                return Some((start, end, next));
            }
        }
        None
    }
}
impl Unit {
    pub fn u8(byte: u8) -> Unit {}
    pub fn eoi(num_byte_equiv_classes: usize) -> Unit {}
    pub fn as_u8(self) -> Option<u8> {}
    pub fn as_eoi(self) -> Option<u16> {}
    pub fn as_usize(self) -> usize {}
    pub fn is_byte(self, byte: u8) -> bool {}
    pub fn is_eoi(self) -> bool {
        self.as_eoi().is_some()
    }
    pub fn is_word_byte(self) -> bool {}
}
