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
pub unsafe trait Automaton {
    fn next_state(&self, current: StateID, input: u8) -> StateID;
    unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID;
    fn next_eoi_state(&self, current: StateID) -> StateID;
    fn start_state(&self, config: &start::Config) -> Result<StateID, StartError>;
    fn start_state_forward(&self, input: &Input<'_>) -> Result<StateID, MatchError>;
    fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError>;
    #[inline]
    fn universal_start_state(&self, _mode: Anchored) -> Option<StateID>;
    fn is_special_state(&self, id: StateID) -> bool;
    fn is_dead_state(&self, id: StateID) -> bool;
    fn is_quit_state(&self, id: StateID) -> bool;
    fn is_match_state(&self, id: StateID) -> bool;
    fn is_start_state(&self, id: StateID) -> bool;
    fn is_accel_state(&self, id: StateID) -> bool;
    fn pattern_len(&self) -> usize;
    fn match_len(&self, id: StateID) -> usize;
    fn match_pattern(&self, id: StateID, index: usize) -> PatternID;
    fn has_empty(&self) -> bool;
    fn is_utf8(&self) -> bool;
    fn is_always_start_anchored(&self) -> bool;
    #[inline]
    fn accelerator(&self, _id: StateID) -> &[u8];
    #[inline]
    fn get_prefilter(&self) -> Option<&Prefilter>;
    #[inline]
    fn try_search_fwd(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>;
    #[inline]
    fn try_search_rev(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>;
    #[inline]
    fn try_search_overlapping_fwd(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError>;
    #[inline]
    fn try_search_overlapping_rev(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError>;
    #[cfg(feature = "alloc")]
    #[inline]
    fn try_which_overlapping_matches(
        &self,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), MatchError>;
}
pub(crate) trait U16 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn high_u8(self) -> u8;
}
pub(crate) trait U32 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn low_u16(self) -> u16;
    fn high_u16(self) -> u16;
}
pub(crate) trait Usize {
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
    fn as_u32(self) -> u32;
    fn as_u64(self) -> u64;
}
pub(crate) trait U8 {
    fn as_usize(self) -> usize;
}
pub(crate) trait U64 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn low_u16(self) -> u16;
    fn low_u32(self) -> u32;
    fn high_u32(self) -> u32;
}
pub(crate) trait I32 {
    fn as_usize(self) -> usize;
    fn to_bits(self) -> u32;
    fn from_bits(n: u32) -> i32;
}
pub(crate) trait Pointer {
    fn as_usize(self) -> usize;
}
pub(crate) struct Minimizer<'a> {
    dfa: &'a mut dense::OwnedDFA,
    in_transitions: Vec<Vec<Vec<StateID>>>,
    partitions: Vec<StateSet>,
    waiting: Vec<StateSet>,
}
#[cfg(feature = "dfa-build")]
impl OwnedDFA {
    pub(crate) fn set_start_state(
        &mut self,
        anchored: Anchored,
        start: Start,
        id: StateID,
    ) {}
    pub(crate) fn set_transition(
        &mut self,
        from: StateID,
        byte: alphabet::Unit,
        to: StateID,
    ) {}
    pub(crate) fn add_empty_state(&mut self) -> Result<StateID, BuildError> {}
    pub(crate) fn swap_states(&mut self, id1: StateID, id2: StateID) {}
    pub(crate) fn remap(&mut self, map: impl Fn(StateID) -> StateID) {}
    pub(crate) fn remap_state(&mut self, id: StateID, map: impl Fn(StateID) -> StateID) {}
    pub(crate) fn truncate_states(&mut self, len: usize) {}
    pub(crate) fn minimize(&mut self) {
        Minimizer::new(self).run();
    }
    pub(crate) fn set_pattern_map(
        &mut self,
        map: &BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<(), BuildError> {}
    pub(crate) fn accelerate(&mut self) {}
    pub(crate) fn shuffle(
        &mut self,
        mut matches: BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<(), BuildError> {}
    fn set_universal_starts(&mut self) {}
}
impl<'a> Minimizer<'a> {
    pub fn new(dfa: &'a mut dense::OwnedDFA) -> Minimizer<'a> {
        let in_transitions = Minimizer::incoming_transitions(dfa);
        let partitions = Minimizer::initial_partitions(dfa);
        let waiting = partitions.clone();
        Minimizer {
            dfa,
            in_transitions,
            partitions,
            waiting,
        }
    }
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
    fn find_waiting(&self, set: &StateSet) -> Option<usize> {}
    fn find_incoming_to(
        &self,
        b: alphabet::Unit,
        set: &StateSet,
        incoming: &mut StateSet,
    ) {}
    fn initial_partitions(dfa: &dense::OwnedDFA) -> Vec<StateSet> {}
    fn incoming_transitions(dfa: &dense::OwnedDFA) -> Vec<Vec<Vec<StateID>>> {}
}
