use core::{cell::RefCell, fmt, mem};
use alloc::{collections::BTreeMap, rc::Rc, vec, vec::Vec};
use crate::{
    dfa::{automaton::Automaton, dense, DEAD},
    util::{alphabet, primitives::{PatternID, StateID}},
};
pub(crate) struct Minimizer<'a> {
    dfa: &'a mut dense::OwnedDFA,
    in_transitions: Vec<Vec<Vec<StateID>>>,
    partitions: Vec<StateSet>,
    waiting: Vec<StateSet>,
}
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
}
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Debug)]
pub(crate) struct StateTransitionIter<'a> {
    len: usize,
    it: iter::Enumerate<slice::Iter<'a, StateID>>,
}
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct StateSet {
    ids: Rc<RefCell<Vec<StateID>>>,
}
impl<'a> Minimizer<'a> {
    pub fn new(dfa: &'a mut dense::OwnedDFA) -> Minimizer<'a> {}
    pub fn run(mut self) {}
    fn find_waiting(&self, set: &StateSet) -> Option<usize> {}
    fn find_incoming_to(
        &self,
        b: alphabet::Unit,
        set: &StateSet,
        incoming: &mut StateSet,
    ) {}
    fn initial_partitions(dfa: &dense::OwnedDFA) -> Vec<StateSet> {}
    fn incoming_transitions(dfa: &dense::OwnedDFA) -> Vec<Vec<Vec<StateID>>> {
        let mut incoming = vec![];
        for _ in dfa.states() {
            incoming.push(vec![vec![]; dfa.alphabet_len()]);
        }
        for state in dfa.states() {
            for (b, next) in state.transitions() {
                incoming[dfa.to_index(next)][b.as_usize()].push(state.id());
            }
        }
        incoming
    }
}
impl<'a> State<'a> {
    pub(crate) fn transitions(&self) -> StateTransitionIter<'_> {
        StateTransitionIter {
            len: self.transitions.len(),
            it: self.transitions.iter().enumerate(),
        }
    }
    pub(crate) fn sparse_transitions(&self) -> StateSparseTransitionIter<'_> {}
    pub(crate) fn id(&self) -> StateID {
        self.id
    }
    #[cfg(feature = "dfa-build")]
    fn accelerate(&self, classes: &ByteClasses) -> Option<Accel> {}
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
    pub fn is_eoi(self) -> bool {}
    pub fn is_word_byte(self) -> bool {}
}
