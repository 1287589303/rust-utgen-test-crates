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
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
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
    ) {
        incoming.clear();
        set.iter(|id| {
            for &inid in &self.in_transitions[self.dfa.to_index(id)][b.as_usize()] {
                incoming.add(inid);
            }
        });
        incoming.canonicalize();
    }
    fn initial_partitions(dfa: &dense::OwnedDFA) -> Vec<StateSet> {}
    fn incoming_transitions(dfa: &dense::OwnedDFA) -> Vec<Vec<Vec<StateID>>> {}
}
impl StateSet {
    fn empty() -> StateSet {}
    fn add(&mut self, id: StateID) {}
    fn min(&self) -> StateID {}
    fn canonicalize(&mut self) {
        self.ids.borrow_mut().sort();
        self.ids.borrow_mut().dedup();
    }
    fn clear(&mut self) {
        self.ids.borrow_mut().clear();
    }
    fn len(&self) -> usize {}
    fn is_empty(&self) -> bool {}
    fn deep_clone(&self) -> StateSet {}
    fn iter<F: FnMut(StateID)>(&self, mut f: F) {
        for &id in self.ids.borrow().iter() {
            f(id);
        }
    }
    fn intersection(&self, other: &StateSet, dest: &mut StateSet) {}
    fn subtract(&self, other: &StateSet, dest: &mut StateSet) {}
}
