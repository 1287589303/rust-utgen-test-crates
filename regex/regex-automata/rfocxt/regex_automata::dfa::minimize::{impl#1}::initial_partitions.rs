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
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct StateSet {
    ids: Rc<RefCell<Vec<StateID>>>,
}
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
    fn initial_partitions(dfa: &dense::OwnedDFA) -> Vec<StateSet> {
        let mut matching: BTreeMap<Vec<PatternID>, StateSet> = BTreeMap::new();
        let mut is_quit = StateSet::empty();
        let mut no_match = StateSet::empty();
        for state in dfa.states() {
            if dfa.is_match_state(state.id()) {
                let mut pids = vec![];
                for i in 0..dfa.match_len(state.id()) {
                    pids.push(dfa.match_pattern(state.id(), i));
                }
                matching.entry(pids).or_insert(StateSet::empty()).add(state.id());
            } else if dfa.is_quit_state(state.id()) {
                is_quit.add(state.id());
            } else {
                no_match.add(state.id());
            }
        }
        let mut sets: Vec<StateSet> = matching.into_iter().map(|(_, set)| set).collect();
        sets.push(no_match);
        sets.push(is_quit);
        sets
    }
    fn incoming_transitions(dfa: &dense::OwnedDFA) -> Vec<Vec<Vec<StateID>>> {}
}
impl StateSet {
    fn empty() -> StateSet {
        StateSet {
            ids: Rc::new(RefCell::new(vec![])),
        }
    }
    fn add(&mut self, id: StateID) {
        self.ids.borrow_mut().push(id);
    }
    fn min(&self) -> StateID {}
    fn canonicalize(&mut self) {}
    fn clear(&mut self) {}
    fn len(&self) -> usize {}
    fn is_empty(&self) -> bool {}
    fn deep_clone(&self) -> StateSet {}
    fn iter<F: FnMut(StateID)>(&self, mut f: F) {}
    fn intersection(&self, other: &StateSet, dest: &mut StateSet) {}
    fn subtract(&self, other: &StateSet, dest: &mut StateSet) {}
}
impl<'a> State<'a> {
    pub(crate) fn transitions(&self) -> StateTransitionIter<'_> {}
    pub(crate) fn sparse_transitions(&self) -> StateSparseTransitionIter<'_> {}
    pub(crate) fn id(&self) -> StateID {
        self.id
    }
    #[cfg(feature = "dfa-build")]
    fn accelerate(&self, classes: &ByteClasses) -> Option<Accel> {}
}
