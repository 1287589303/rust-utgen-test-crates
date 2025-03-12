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
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct StateSet {
    ids: Rc<RefCell<Vec<StateID>>>,
}
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Special {
    /// The identifier of the last special state in a DFA. A state is special
    /// if and only if its identifier is less than or equal to `max`.
    pub(crate) max: StateID,
    /// The identifier of the quit state in a DFA. (There is no analogous field
    /// for the dead state since the dead state's ID is always zero, regardless
    /// of state ID size.)
    pub(crate) quit_id: StateID,
    /// The identifier of the first match state.
    pub(crate) min_match: StateID,
    /// The identifier of the last match state.
    pub(crate) max_match: StateID,
    /// The identifier of the first accelerated state.
    pub(crate) min_accel: StateID,
    /// The identifier of the last accelerated state.
    pub(crate) max_accel: StateID,
    /// The identifier of the first start state.
    pub(crate) min_start: StateID,
    /// The identifier of the last start state.
    pub(crate) max_start: StateID,
}
pub(crate) struct StartStateIter<'a> {
    st: StartTable<&'a [u32]>,
    i: usize,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Debug)]
pub struct ByteClassIter<'a> {
    classes: &'a ByteClasses,
    i: usize,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Start {
    /// This occurs when the starting position is not any of the ones below.
    NonWordByte = 0,
    /// This occurs when the byte immediately preceding the start of the search
    /// is an ASCII word byte.
    WordByte = 1,
    /// This occurs when the starting position of the search corresponds to the
    /// beginning of the haystack.
    Text = 2,
    /// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\n`.
    LineLF = 3,
    /// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\r`.
    LineCR = 4,
    /// This occurs when a custom line terminator has been set via a
    /// `LookMatcher`, and when that line terminator is neither a `\r` or a
    /// `\n`.
    ///
    /// If the custom line terminator is a word byte, then this start
    /// configuration is still selected. DFAs that implement word boundary
    /// assertions will likely need to check whether the custom line terminator
    /// is a word byte, in which case, it should behave as if the byte
    /// satisfies `\b` in addition to multi-line anchors.
    CustomLineTerminator = 5,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Anchored {
    /// Run an unanchored search. This means a match may occur anywhere at or
    /// after the start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    No,
    /// Run an anchored search. This means that a match must begin at the
    /// start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    Yes,
    /// Run an anchored search for a specific pattern. This means that a match
    /// must be for the given pattern and must begin at the start position of
    /// the search.
    Pattern(PatternID),
}
impl<'a> Minimizer<'a> {
    pub fn new(dfa: &'a mut dense::OwnedDFA) -> Minimizer<'a> {}
    pub fn run(mut self) {
        let stride2 = self.dfa.stride2();
        let as_state_id = |index: usize| -> StateID {
            StateID::new(index << stride2).unwrap()
        };
        let as_index = |id: StateID| -> usize { id.as_usize() >> stride2 };
        let mut incoming = StateSet::empty();
        let mut scratch1 = StateSet::empty();
        let mut scratch2 = StateSet::empty();
        let mut newparts = vec![];
        while let Some(set) = self.waiting.pop() {
            for b in self.dfa.byte_classes().iter() {
                self.find_incoming_to(b, &set, &mut incoming);
                if incoming.is_empty() {
                    continue;
                }
                for p in 0..self.partitions.len() {
                    self.partitions[p].intersection(&incoming, &mut scratch1);
                    if scratch1.is_empty() {
                        newparts.push(self.partitions[p].clone());
                        continue;
                    }
                    self.partitions[p].subtract(&incoming, &mut scratch2);
                    if scratch2.is_empty() {
                        newparts.push(self.partitions[p].clone());
                        continue;
                    }
                    let (x, y) = (scratch1.deep_clone(), scratch2.deep_clone());
                    newparts.push(x.clone());
                    newparts.push(y.clone());
                    match self.find_waiting(&self.partitions[p]) {
                        Some(i) => {
                            self.waiting[i] = x;
                            self.waiting.push(y);
                        }
                        None => {
                            if x.len() <= y.len() {
                                self.waiting.push(x);
                            } else {
                                self.waiting.push(y);
                            }
                        }
                    }
                }
                newparts = mem::replace(&mut self.partitions, newparts);
                newparts.clear();
            }
        }
        let mut state_to_part = vec![DEAD; self.dfa.state_len()];
        for p in &self.partitions {
            p.iter(|id| state_to_part[as_index(id)] = p.min());
        }
        let mut minimal_ids = vec![DEAD; self.dfa.state_len()];
        let mut new_index = 0;
        for state in self.dfa.states() {
            if state_to_part[as_index(state.id())] == state.id() {
                minimal_ids[as_index(state.id())] = as_state_id(new_index);
                new_index += 1;
            }
        }
        let minimal_count = new_index;
        let remap = |old| minimal_ids[as_index(state_to_part[as_index(old)])];
        for id in (0..self.dfa.state_len()).map(as_state_id) {
            if state_to_part[as_index(id)] != id {
                continue;
            }
            self.dfa.remap_state(id, remap);
            self.dfa.swap_states(id, minimal_ids[as_index(id)]);
        }
        self.dfa.truncate_states(minimal_count);
        let starts: Vec<_> = self.dfa.starts().collect();
        for (old_start_id, anchored, start_type) in starts {
            self.dfa.set_start_state(anchored, start_type, remap(old_start_id));
        }
        let mut pmap = BTreeMap::new();
        for (match_id, pattern_ids) in self.dfa.pattern_map() {
            let new_id = remap(match_id);
            pmap.insert(new_id, pattern_ids);
        }
        self.dfa.set_pattern_map(&pmap).unwrap();
        let old = self.dfa.special().clone();
        let new = self.dfa.special_mut();
        if old.matches() {
            new.min_match = StateID::MAX;
            new.max_match = StateID::ZERO;
            for i in as_index(old.min_match)..=as_index(old.max_match) {
                let new_id = remap(as_state_id(i));
                if new_id < new.min_match {
                    new.min_match = new_id;
                }
                if new_id > new.max_match {
                    new.max_match = new_id;
                }
            }
        }
        if old.starts() {
            new.min_start = StateID::MAX;
            new.max_start = StateID::ZERO;
            for i in as_index(old.min_start)..=as_index(old.max_start) {
                let new_id = remap(as_state_id(i));
                if new_id == DEAD {
                    continue;
                }
                if new_id < new.min_start {
                    new.min_start = new_id;
                }
                if new_id > new.max_start {
                    new.max_start = new_id;
                }
            }
            if new.max_start == DEAD {
                new.min_start = DEAD;
            }
        }
        new.quit_id = remap(new.quit_id);
        new.set_max();
    }
    fn find_waiting(&self, set: &StateSet) -> Option<usize> {
        self.waiting.iter().position(|s| s == set)
    }
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
impl ByteClasses {
    #[inline]
    pub fn empty() -> ByteClasses {}
    #[inline]
    pub fn singletons() -> ByteClasses {}
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteClasses, usize), DeserializeError> {}
    pub(crate) fn write_to(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
    #[inline]
    pub fn set(&mut self, byte: u8, class: u8) {}
    #[inline]
    pub fn get(&self, byte: u8) -> u8 {}
    #[inline]
    pub fn get_by_unit(&self, unit: Unit) -> usize {}
    #[inline]
    pub fn eoi(&self) -> Unit {}
    #[inline]
    pub fn alphabet_len(&self) -> usize {}
    #[inline]
    pub fn stride2(&self) -> usize {}
    #[inline]
    pub fn is_singleton(&self) -> bool {}
    #[inline]
    pub fn iter(&self) -> ByteClassIter<'_> {
        ByteClassIter {
            classes: self,
            i: 0,
        }
    }
    pub fn representatives<R: core::ops::RangeBounds<u8>>(
        &self,
        range: R,
    ) -> ByteClassRepresentatives<'_> {}
    #[inline]
    pub fn elements(&self, class: Unit) -> ByteClassElements {}
    fn element_ranges(&self, class: Unit) -> ByteClassElementRanges {}
}
impl StateSet {
    fn empty() -> StateSet {
        StateSet {
            ids: Rc::new(RefCell::new(vec![])),
        }
    }
    fn add(&mut self, id: StateID) {}
    fn min(&self) -> StateID {}
    fn canonicalize(&mut self) {}
    fn clear(&mut self) {}
    fn len(&self) -> usize {
        self.ids.borrow().len()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn deep_clone(&self) -> StateSet {
        let ids = self.ids.borrow().iter().cloned().collect();
        StateSet {
            ids: Rc::new(RefCell::new(ids)),
        }
    }
    fn iter<F: FnMut(StateID)>(&self, mut f: F) {
        for &id in self.ids.borrow().iter() {
            f(id);
        }
    }
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
    fn subtract(&self, other: &StateSet, dest: &mut StateSet) {
        dest.clear();
        if self.is_empty() || other.is_empty() {
            self.iter(|s| dest.add(s));
            return;
        }
        let (seta, setb) = (self.ids.borrow(), other.ids.borrow());
        let (mut ita, mut itb) = (seta.iter().cloned(), setb.iter().cloned());
        let (mut a, mut b) = (ita.next().unwrap(), itb.next().unwrap());
        loop {
            if a == b {
                a = match ita.next() {
                    None => break,
                    Some(a) => a,
                };
                b = match itb.next() {
                    None => {
                        dest.add(a);
                        break;
                    }
                    Some(b) => b,
                };
            } else if a < b {
                dest.add(a);
                a = match ita.next() {
                    None => break,
                    Some(a) => a,
                };
            } else {
                b = match itb.next() {
                    None => {
                        dest.add(a);
                        break;
                    }
                    Some(b) => b,
                };
            }
        }
        for a in ita {
            dest.add(a);
        }
    }
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
impl Special {
    #[cfg(feature = "dfa-build")]
    pub(crate) fn new() -> Special {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn remap(&self, map: impl Fn(StateID) -> StateID) -> Special {}
    pub(crate) fn from_bytes(
        mut slice: &[u8],
    ) -> Result<(Special, usize), DeserializeError> {}
    pub(crate) fn validate(&self) -> Result<(), DeserializeError> {}
    pub(crate) fn validate_state_len(
        &self,
        len: usize,
        stride2: usize,
    ) -> Result<(), DeserializeError> {}
    pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn set_max(&mut self) {
        use core::cmp::max;
        self.max = max(
            self.quit_id,
            max(self.max_match, max(self.max_accel, self.max_start)),
        );
    }
    #[cfg(feature = "dfa-build")]
    pub(crate) fn set_no_special_start_states(&mut self) {}
    #[inline]
    pub(crate) fn is_special_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_dead_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_quit_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_match_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_accel_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_start_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn match_len(&self, stride: usize) -> usize {}
    #[inline]
    pub(crate) fn matches(&self) -> bool {
        self.min_match != DEAD
    }
    #[cfg(feature = "dfa-build")]
    pub(crate) fn accel_len(&self, stride: usize) -> usize {}
    #[inline]
    pub(crate) fn accels(&self) -> bool {}
    #[inline]
    pub(crate) fn starts(&self) -> bool {
        self.min_start != DEAD
    }
}
