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
    fn add(&mut self, id: StateID) {
        self.ids.borrow_mut().push(id);
    }
    fn min(&self) -> StateID {}
    fn canonicalize(&mut self) {}
    fn clear(&mut self) {
        self.ids.borrow_mut().clear();
    }
    fn len(&self) -> usize {}
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn deep_clone(&self) -> StateSet {}
    fn iter<F: FnMut(StateID)>(&self, mut f: F) {}
    fn intersection(&self, other: &StateSet, dest: &mut StateSet) {
        dest.clear();
        if self.is_empty() || other.is_empty() {
            return;
        }
        let (seta, setb) = (self.ids.borrow(), other.ids.borrow());
        let (mut ita, mut itb) = (seta.iter().cloned(), setb.iter().cloned());
        let (mut a, mut b) = (ita.next().unwrap(), itb.next().unwrap());
        loop {
            if a == b {
                dest.add(a);
                a = match ita.next() {
                    None => break,
                    Some(a) => a,
                };
                b = match itb.next() {
                    None => break,
                    Some(b) => b,
                };
            } else if a < b {
                a = match ita.next() {
                    None => break,
                    Some(a) => a,
                };
            } else {
                b = match itb.next() {
                    None => break,
                    Some(b) => b,
                };
            }
        }
    }
    fn subtract(&self, other: &StateSet, dest: &mut StateSet) {}
}
