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
#[derive(Clone, Debug)]
pub struct DFA {
    config: Config,
    nfa: thompson::NFA,
    stride2: usize,
    start_map: StartByteMap,
    classes: ByteClasses,
    quitset: ByteSet,
    cache_capacity: usize,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
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
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    crlf: bool,
    line_terminator: u8,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    utf8: bool,
    nest_limit: u32,
    octal: bool,
}
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    utf8: Option<bool>,
    reverse: Option<bool>,
    nfa_size_limit: Option<Option<usize>>,
    shrink: Option<bool>,
    which_captures: Option<WhichCaptures>,
    look_matcher: Option<LookMatcher>,
    #[cfg(test)]
    unanchored_prefix: Option<bool>,
}
#[derive(Clone)]
pub(crate) struct StartByteMap {
    map: [Start; 256],
}
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug, Default)]
pub struct Config {
    accelerate: Option<bool>,
    pre: Option<Option<Prefilter>>,
    minimize: Option<bool>,
    match_kind: Option<MatchKind>,
    start_kind: Option<StartKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    specialize_start_states: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    determinize_size_limit: Option<Option<usize>>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    utf8_empty: Option<bool>,
    autopre: Option<bool>,
    pre: Option<Option<Prefilter>>,
    which_captures: Option<WhichCaptures>,
    nfa_size_limit: Option<Option<usize>>,
    onepass_size_limit: Option<Option<usize>>,
    hybrid_cache_capacity: Option<usize>,
    hybrid: Option<bool>,
    dfa: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    dfa_state_limit: Option<Option<usize>>,
    onepass: Option<bool>,
    backtrack: Option<bool>,
    byte_classes: Option<bool>,
    line_terminator: Option<u8>,
}
impl DFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<DFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError> {}
    pub fn always_match() -> Result<DFA, BuildError> {}
    pub fn never_match() -> Result<DFA, BuildError> {
        let nfa = thompson::NFA::never_match();
        Builder::new().build_from_nfa(nfa)
    }
    pub fn config() -> Config {}
    pub fn builder() -> Builder {}
    pub fn create_cache(&self) -> Cache {}
    pub fn reset_cache(&self, cache: &mut Cache) {}
    pub fn pattern_len(&self) -> usize {}
    pub fn byte_classes(&self) -> &ByteClasses {}
    pub fn get_config(&self) -> &Config {}
    pub fn get_nfa(&self) -> &thompson::NFA {}
    fn stride2(&self) -> usize {}
    fn stride(&self) -> usize {}
    pub fn memory_usage(&self) -> usize {}
}
impl NFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<NFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError> {}
    pub fn always_match() -> NFA {}
    pub fn never_match() -> NFA {
        let mut builder = Builder::new();
        let sid = builder.add_fail().unwrap();
        builder.build(sid, sid).unwrap()
    }
    #[cfg(feature = "syntax")]
    pub fn config() -> Config {}
    #[cfg(feature = "syntax")]
    pub fn compiler() -> Compiler {}
    pub fn patterns(&self) -> PatternIter<'_> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {}
    #[inline]
    pub fn start_anchored(&self) -> StateID {}
    #[inline]
    pub fn start_unanchored(&self) -> StateID {}
    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {}
    #[inline]
    pub(crate) fn byte_class_set(&self) -> &ByteClassSet {}
    #[inline]
    pub fn byte_classes(&self) -> &ByteClasses {}
    #[inline]
    pub fn state(&self, id: StateID) -> &State {}
    #[inline]
    pub fn states(&self) -> &[State] {}
    #[inline]
    pub fn group_info(&self) -> &GroupInfo {}
    #[inline]
    pub fn has_capture(&self) -> bool {}
    #[inline]
    pub fn has_empty(&self) -> bool {}
    #[inline]
    pub fn is_utf8(&self) -> bool {}
    #[inline]
    pub fn is_reverse(&self) -> bool {}
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {}
    #[inline]
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl Builder {
    pub fn new() -> Builder {
        Builder {
            config: Config::default(),
            #[cfg(feature = "syntax")]
            thompson: thompson::Compiler::new(),
        }
    }
    #[cfg(feature = "syntax")]
    pub fn build(&self, pattern: &str) -> Result<DFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError> {}
    pub fn build_from_nfa(&self, nfa: thompson::NFA) -> Result<DFA, BuildError> {
        let quitset = self.config.quit_set_from_nfa(&nfa)?;
        let classes = self.config.byte_classes_from_nfa(&nfa, &quitset);
        let min_cache = minimum_cache_capacity(
            &nfa,
            &classes,
            self.config.get_starts_for_each_pattern(),
        );
        let mut cache_capacity = self.config.get_cache_capacity();
        if cache_capacity < min_cache {
            if self.config.get_skip_cache_capacity_check() {
                debug!(
                    "given capacity ({}) is too small, \
                     since skip_cache_capacity_check is enabled, \
                     setting cache capacity to minimum ({})",
                    cache_capacity, min_cache,
                );
                cache_capacity = min_cache;
            } else {
                return Err(
                    BuildError::insufficient_cache_capacity(min_cache, cache_capacity),
                );
            }
        }
        if let Err(err) = minimum_lazy_state_id(&classes) {
            return Err(BuildError::insufficient_state_id_capacity(err));
        }
        let stride2 = classes.stride2();
        let start_map = StartByteMap::new(nfa.look_matcher());
        Ok(DFA {
            config: self.config.clone(),
            nfa,
            stride2,
            start_map,
            classes,
            quitset,
            cache_capacity,
        })
    }
    pub fn configure(&mut self, config: Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn thompson(&mut self, config: thompson::Config) -> &mut Builder {}
}
