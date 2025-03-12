use core::{cell::RefCell, fmt, mem};
use alloc::{collections::BTreeMap, rc::Rc, vec, vec::Vec};
use crate::{
    dfa::{automaton::Automaton, dense, DEAD},
    util::{alphabet, primitives::{PatternID, StateID}},
};
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct StateSet {
    ids: Rc<RefCell<Vec<StateID>>>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl StateSet {
    fn empty() -> StateSet {}
    fn add(&mut self, id: StateID) {}
    fn min(&self) -> StateID {}
    fn canonicalize(&mut self) {}
    fn clear(&mut self) {}
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
