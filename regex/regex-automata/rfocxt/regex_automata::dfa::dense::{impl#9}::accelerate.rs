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
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
}
#[derive(Debug)]
pub struct DeserializeError(DeserializeErrorKind);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone)]
pub(crate) struct Accel {
    /// The first byte is the length. Subsequent bytes are the accelerated
    /// bytes.
    ///
    /// Note that we make every accelerator 8 bytes as a slightly wasteful
    /// way of making sure alignment is always correct for state ID sizes of
    /// 1, 2, 4 and 8. This should be okay since accelerated states aren't
    /// particularly common, especially when Unicode is enabled.
    bytes: [u8; ACCEL_CAP],
}
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Debug)]
pub(super) struct Remapper {
    /// A map from the index of a state to its pre-multiplied identifier.
    ///
    /// When a state is swapped with another, then their corresponding
    /// locations in this map are also swapped. Thus, its new position will
    /// still point to its old pre-multiplied StateID.
    ///
    /// While there is a bit more to it, this then allows us to rewrite the
    /// state IDs in a DFA's transition table in a single pass. This is done
    /// by iterating over every ID in this map, then iterating over each
    /// transition for the state at that ID and re-mapping the transition from
    /// `old_id` to `map[dfa.to_index(old_id)]`. That is, we find the position
    /// in this map where `old_id` *started*, and set it to where it ended up
    /// after all swaps have been completed.
    map: Vec<StateID>,
    /// A mapper from state index to state ID (and back).
    idxmap: IndexMapper,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
    pub(crate) fn minimize(&mut self) {}
    pub(crate) fn set_pattern_map(
        &mut self,
        map: &BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<(), BuildError> {}
    pub(crate) fn accelerate(&mut self) {
        if self.state_len() <= 2 {
            return;
        }
        let mut accels = BTreeMap::new();
        let (mut cmatch, mut cstart, mut cnormal) = (0, 0, 0);
        for state in self.states() {
            if let Some(accel) = state.accelerate(self.byte_classes()) {
                debug!(
                    "accelerating full DFA state {}: {:?}", state.id().as_usize(), accel,
                );
                accels.insert(state.id(), accel);
                if self.is_match_state(state.id()) {
                    cmatch += 1;
                } else if self.is_start_state(state.id()) {
                    cstart += 1;
                } else {
                    assert!(! self.is_dead_state(state.id()));
                    assert!(! self.is_quit_state(state.id()));
                    cnormal += 1;
                }
            }
        }
        if accels.is_empty() {
            return;
        }
        let original_accels_len = accels.len();
        let mut remapper = Remapper::new(self);
        let mut new_matches = self.ms.to_map(self);
        self.special.min_accel = StateID::MAX;
        self.special.max_accel = StateID::ZERO;
        let update_special_accel = |special: &mut Special, accel_id: StateID| {
            special.min_accel = cmp::min(special.min_accel, accel_id);
            special.max_accel = cmp::max(special.max_accel, accel_id);
        };
        if cmatch > 0 && self.special.matches() {
            let mut next_id = self.special.max_match;
            let mut cur_id = next_id;
            while cur_id >= self.special.min_match {
                if let Some(accel) = accels.remove(&cur_id) {
                    accels.insert(next_id, accel);
                    update_special_accel(&mut self.special, next_id);
                    if cur_id != next_id {
                        remapper.swap(self, cur_id, next_id);
                        let cur_pids = new_matches.remove(&cur_id).unwrap();
                        let next_pids = new_matches.remove(&next_id).unwrap();
                        new_matches.insert(cur_id, next_pids);
                        new_matches.insert(next_id, cur_pids);
                    }
                    next_id = self.tt.prev_state_id(next_id);
                }
                cur_id = self.tt.prev_state_id(cur_id);
            }
        }
        if cnormal > 0 {
            let mut next_start_id = self.special.min_start;
            let mut cur_id = self.to_state_id(self.state_len() - 1);
            let mut next_norm_id = self.tt.next_state_id(self.special.max_start);
            while cur_id >= next_norm_id {
                if let Some(accel) = accels.remove(&cur_id) {
                    remapper.swap(self, next_start_id, cur_id);
                    remapper.swap(self, next_norm_id, cur_id);
                    if let Some(accel2) = accels.remove(&next_norm_id) {
                        accels.insert(cur_id, accel2);
                    }
                    if let Some(accel2) = accels.remove(&next_start_id) {
                        accels.insert(next_norm_id, accel2);
                    }
                    accels.insert(next_start_id, accel);
                    update_special_accel(&mut self.special, next_start_id);
                    self.special.min_start = self
                        .tt
                        .next_state_id(self.special.min_start);
                    self.special.max_start = self
                        .tt
                        .next_state_id(self.special.max_start);
                    next_start_id = self.tt.next_state_id(next_start_id);
                    next_norm_id = self.tt.next_state_id(next_norm_id);
                }
                if !accels.contains_key(&cur_id) {
                    cur_id = self.tt.prev_state_id(cur_id);
                }
            }
        }
        if cstart > 0 {
            let mut next_id = self.special.min_start;
            let mut cur_id = next_id;
            while cur_id <= self.special.max_start {
                if let Some(accel) = accels.remove(&cur_id) {
                    remapper.swap(self, cur_id, next_id);
                    accels.insert(next_id, accel);
                    update_special_accel(&mut self.special, next_id);
                    next_id = self.tt.next_state_id(next_id);
                }
                cur_id = self.tt.next_state_id(cur_id);
            }
        }
        remapper.remap(self);
        self.set_pattern_map(&new_matches).unwrap();
        self.special.set_max();
        self.special.validate().expect("special state ranges should validate");
        self.special
            .validate_state_len(self.state_len(), self.stride2())
            .expect("special state ranges should be consistent with state length");
        assert_eq!(
            self.special.accel_len(self.stride()), original_accels_len,
            "mismatch with expected number of accelerated states",
        );
        let mut prev: Option<StateID> = None;
        for (id, accel) in accels {
            assert!(prev.map_or(true, | p | self.tt.next_state_id(p) == id));
            prev = Some(id);
            self.accels.add(accel);
        }
    }
    pub(crate) fn shuffle(
        &mut self,
        mut matches: BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<(), BuildError> {}
    fn set_universal_starts(&mut self) {}
}
impl Special {
    #[cfg(feature = "dfa-build")]
    pub(crate) fn new() -> Special {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn remap(&self, map: impl Fn(StateID) -> StateID) -> Special {}
    pub(crate) fn from_bytes(
        mut slice: &[u8],
    ) -> Result<(Special, usize), DeserializeError> {}
    pub(crate) fn validate(&self) -> Result<(), DeserializeError> {
        if self.min_match == DEAD && self.max_match != DEAD {
            err!("min_match is DEAD, but max_match is not");
        }
        if self.min_match != DEAD && self.max_match == DEAD {
            err!("max_match is DEAD, but min_match is not");
        }
        if self.min_accel == DEAD && self.max_accel != DEAD {
            err!("min_accel is DEAD, but max_accel is not");
        }
        if self.min_accel != DEAD && self.max_accel == DEAD {
            err!("max_accel is DEAD, but min_accel is not");
        }
        if self.min_start == DEAD && self.max_start != DEAD {
            err!("min_start is DEAD, but max_start is not");
        }
        if self.min_start != DEAD && self.max_start == DEAD {
            err!("max_start is DEAD, but min_start is not");
        }
        if self.min_match > self.max_match {
            err!("min_match should not be greater than max_match");
        }
        if self.min_accel > self.max_accel {
            err!("min_accel should not be greater than max_accel");
        }
        if self.min_start > self.max_start {
            err!("min_start should not be greater than max_start");
        }
        if self.matches() && self.quit_id >= self.min_match {
            err!("quit_id should not be greater than min_match");
        }
        if self.accels() && self.quit_id >= self.min_accel {
            err!("quit_id should not be greater than min_accel");
        }
        if self.starts() && self.quit_id >= self.min_start {
            err!("quit_id should not be greater than min_start");
        }
        if self.matches() && self.accels() && self.min_accel < self.min_match {
            err!("min_match should not be greater than min_accel");
        }
        if self.matches() && self.starts() && self.min_start < self.min_match {
            err!("min_match should not be greater than min_start");
        }
        if self.accels() && self.starts() && self.min_start < self.min_accel {
            err!("min_accel should not be greater than min_start");
        }
        if self.max < self.quit_id {
            err!("quit_id should not be greater than max");
        }
        if self.max < self.max_match {
            err!("max_match should not be greater than max");
        }
        if self.max < self.max_accel {
            err!("max_accel should not be greater than max");
        }
        if self.max < self.max_start {
            err!("max_start should not be greater than max");
        }
        Ok(())
    }
    pub(crate) fn validate_state_len(
        &self,
        len: usize,
        stride2: usize,
    ) -> Result<(), DeserializeError> {
        if (self.max.as_usize() >> stride2) >= len {
            err!("max should not be greater than or equal to state length");
        }
        Ok(())
    }
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
    pub(crate) fn accel_len(&self, stride: usize) -> usize {
        if self.accels() {
            (self.max_accel.as_usize() - self.min_accel.as_usize() + stride) / stride
        } else {
            0
        }
    }
    #[inline]
    pub(crate) fn accels(&self) -> bool {}
    #[inline]
    pub(crate) fn starts(&self) -> bool {}
}
impl<'a> State<'a> {
    pub(crate) fn transitions(&self) -> StateTransitionIter<'_> {}
    pub(crate) fn sparse_transitions(&self) -> StateSparseTransitionIter<'_> {}
    pub(crate) fn id(&self) -> StateID {
        self.id
    }
    #[cfg(feature = "dfa-build")]
    fn accelerate(&self, classes: &ByteClasses) -> Option<Accel> {
        let mut accel = Accel::new();
        for (class, id) in self.transitions() {
            if id == self.id() {
                continue;
            }
            for unit in classes.elements(class) {
                if let Some(byte) = unit.as_u8() {
                    if !accel.add(byte) {
                        return None;
                    }
                }
            }
        }
        if accel.is_empty() { None } else { Some(accel) }
    }
}
