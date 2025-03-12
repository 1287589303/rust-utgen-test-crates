#[cfg(feature = "std")]
type StateMap = std::collections::HashMap<State, LazyStateID>;
#[cfg(not(feature = "std"))]
type StateMap = alloc::collections::BTreeMap<State, LazyStateID>;
use core::{iter, mem::size_of};
use alloc::vec::Vec;
use crate::{
    hybrid::{
        error::{BuildError, CacheError, StartError},
        id::{LazyStateID, LazyStateIDError},
        search,
    },
    nfa::thompson,
    util::{
        alphabet::{self, ByteClasses, ByteSet},
        determinize::{self, State, StateBuilderEmpty, StateBuilderNFA},
        empty, prefilter::Prefilter, primitives::{PatternID, StateID as NFAStateID},
        search::{Anchored, HalfMatch, Input, MatchError, MatchKind, PatternSet},
        sparse_set::SparseSets, start::{self, Start, StartByteMap},
    },
};
const MIN_STATES: usize = SENTINEL_STATES + 2;
const SENTINEL_STATES: usize = 3;
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    specialize_start_states: Option<bool>,
    cache_capacity: Option<usize>,
    skip_cache_capacity_check: Option<bool>,
    minimum_cache_clear_count: Option<Option<usize>>,
    minimum_bytes_per_state: Option<Option<usize>>,
}
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Debug)]
pub struct Prefilter {
    #[cfg(not(feature = "alloc"))]
    _unused: (),
    #[cfg(feature = "alloc")]
    pre: Arc<dyn PrefilterI>,
    #[cfg(feature = "alloc")]
    is_fast: bool,
    #[cfg(feature = "alloc")]
    max_needle_len: usize,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchKind {
    /// Report all possible matches.
    All,
    /// Report only the leftmost matches. When multiple leftmost matches exist,
    /// report the match corresponding to the part of the regex that appears
    /// first in the syntax.
    LeftmostFirst,
}
impl Config {
    pub fn new() -> Config {}
    pub fn match_kind(mut self, kind: MatchKind) -> Config {}
    pub fn prefilter(mut self, pre: Option<Prefilter>) -> Config {}
    pub fn starts_for_each_pattern(mut self, yes: bool) -> Config {}
    pub fn byte_classes(mut self, yes: bool) -> Config {}
    pub fn unicode_word_boundary(mut self, yes: bool) -> Config {}
    pub fn quit(mut self, byte: u8, yes: bool) -> Config {}
    pub fn specialize_start_states(mut self, yes: bool) -> Config {}
    pub fn cache_capacity(mut self, bytes: usize) -> Config {}
    pub fn skip_cache_capacity_check(mut self, yes: bool) -> Config {}
    pub fn minimum_cache_clear_count(mut self, min: Option<usize>) -> Config {}
    pub fn minimum_bytes_per_state(mut self, min: Option<usize>) -> Config {}
    pub fn get_match_kind(&self) -> MatchKind {}
    pub fn get_prefilter(&self) -> Option<&Prefilter> {}
    pub fn get_starts_for_each_pattern(&self) -> bool {
        self.starts_for_each_pattern.unwrap_or(false)
    }
    pub fn get_byte_classes(&self) -> bool {}
    pub fn get_unicode_word_boundary(&self) -> bool {}
    pub fn get_quit(&self, byte: u8) -> bool {}
    pub fn get_specialize_start_states(&self) -> bool {}
    pub fn get_cache_capacity(&self) -> usize {}
    pub fn get_skip_cache_capacity_check(&self) -> bool {}
    pub fn get_minimum_cache_clear_count(&self) -> Option<usize> {}
    pub fn get_minimum_bytes_per_state(&self) -> Option<usize> {}
    pub fn get_minimum_cache_capacity(
        &self,
        nfa: &thompson::NFA,
    ) -> Result<usize, BuildError> {
        let quitset = self.quit_set_from_nfa(nfa)?;
        let classes = self.byte_classes_from_nfa(nfa, &quitset);
        let starts = self.get_starts_for_each_pattern();
        Ok(minimum_cache_capacity(nfa, &classes, starts))
    }
    fn byte_classes_from_nfa(&self, nfa: &thompson::NFA, quit: &ByteSet) -> ByteClasses {
        if !self.get_byte_classes() {
            ByteClasses::singletons()
        } else {
            let mut set = nfa.byte_class_set().clone();
            if !quit.is_empty() {
                set.add_set(&quit);
            }
            set.byte_classes()
        }
    }
    fn quit_set_from_nfa(&self, nfa: &thompson::NFA) -> Result<ByteSet, BuildError> {
        let mut quit = self.quitset.unwrap_or(ByteSet::empty());
        if nfa.look_set_any().contains_word_unicode() {
            if self.get_unicode_word_boundary() {
                for b in 0x80..=0xFF {
                    quit.add(b);
                }
            } else {
                if !quit.contains_range(0x80, 0xFF) {
                    return Err(BuildError::unsupported_dfa_word_boundary_unicode());
                }
            }
        }
        Ok(quit)
    }
    fn overwrite(&self, o: Config) -> Config {}
}
fn minimum_cache_capacity(
    nfa: &thompson::NFA,
    classes: &ByteClasses,
    starts_for_each_pattern: bool,
) -> usize {
    const ID_SIZE: usize = size_of::<LazyStateID>();
    const STATE_SIZE: usize = size_of::<State>();
    let stride = 1 << classes.stride2();
    let states_len = nfa.states().len();
    let sparses = 2 * states_len * NFAStateID::SIZE;
    let trans = MIN_STATES * stride * ID_SIZE;
    let mut starts = Start::len() * ID_SIZE;
    if starts_for_each_pattern {
        starts += (Start::len() * nfa.pattern_len()) * ID_SIZE;
    }
    assert!(MIN_STATES >= 5, "minimum number of states has to be at least 5");
    let non_sentinel = MIN_STATES.checked_sub(SENTINEL_STATES).unwrap();
    let dead_state_size = State::dead().memory_usage();
    let max_state_size = 5 + 4 + (nfa.pattern_len() * 4) + (states_len * 5);
    let states = (SENTINEL_STATES * (STATE_SIZE + dead_state_size))
        + (non_sentinel * (STATE_SIZE + max_state_size));
    let states_to_sid = (MIN_STATES * STATE_SIZE) + (MIN_STATES * ID_SIZE);
    let stack = states_len * NFAStateID::SIZE;
    let scratch_state_builder = max_state_size;
    trans + starts + states + states_to_sid + sparses + stack + scratch_state_builder
}
