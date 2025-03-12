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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct StateSet {
    ids: Rc<RefCell<Vec<StateID>>>,
}
impl<'a> Minimizer<'a> {
    pub fn new(dfa: &'a mut dense::OwnedDFA) -> Minimizer<'a> {}
    pub fn run(mut self) {}
    fn find_waiting(&self, set: &StateSet) -> Option<usize> {
        self.waiting.iter().position(|s| s == set)
    }
    fn find_incoming_to(
        &self,
        b: alphabet::Unit,
        set: &StateSet,
        incoming: &mut StateSet,
    ) {}
    fn initial_partitions(dfa: &dense::OwnedDFA) -> Vec<StateSet> {}
    fn incoming_transitions(dfa: &dense::OwnedDFA) -> Vec<Vec<Vec<StateID>>> {}
}
