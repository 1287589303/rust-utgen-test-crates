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
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    #[cfg(feature = "syntax")]
    thompson: thompson::Compiler,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[cfg(feature = "dfa-build")]
impl OwnedDFA {
    pub fn always_match() -> Result<OwnedDFA, BuildError> {}
    pub fn never_match() -> Result<OwnedDFA, BuildError> {
        let nfa = thompson::NFA::never_match();
        Builder::new().build_from_nfa(&nfa)
    }
    fn initial(
        classes: ByteClasses,
        pattern_len: usize,
        starts: StartKind,
        lookm: &LookMatcher,
        starts_for_each_pattern: bool,
        pre: Option<Prefilter>,
        quitset: ByteSet,
        flags: Flags,
    ) -> Result<OwnedDFA, BuildError> {}
}
#[cfg(feature = "dfa-build")]
impl Builder {
    pub fn new() -> Builder {
        Builder {
            config: Config::default(),
            #[cfg(feature = "syntax")]
            thompson: thompson::Compiler::new(),
        }
    }
    #[cfg(feature = "syntax")]
    pub fn build(&self, pattern: &str) -> Result<OwnedDFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<OwnedDFA, BuildError> {}
    pub fn build_from_nfa(&self, nfa: &thompson::NFA) -> Result<OwnedDFA, BuildError> {
        let mut quitset = self.config.quitset.unwrap_or(ByteSet::empty());
        if self.config.get_unicode_word_boundary()
            && nfa.look_set_any().contains_word_unicode()
        {
            for b in 0x80..=0xFF {
                quitset.add(b);
            }
        }
        let classes = if !self.config.get_byte_classes() {
            ByteClasses::singletons()
        } else {
            let mut set = nfa.byte_class_set().clone();
            if !quitset.is_empty() {
                set.add_set(&quitset);
            }
            set.byte_classes()
        };
        let mut dfa = DFA::initial(
            classes,
            nfa.pattern_len(),
            self.config.get_starts(),
            nfa.look_matcher(),
            self.config.get_starts_for_each_pattern(),
            self.config.get_prefilter().map(|p| p.clone()),
            quitset,
            Flags::from_nfa(&nfa),
        )?;
        determinize::Config::new()
            .match_kind(self.config.get_match_kind())
            .quit(quitset)
            .dfa_size_limit(self.config.get_dfa_size_limit())
            .determinize_size_limit(self.config.get_determinize_size_limit())
            .run(nfa, &mut dfa)?;
        if self.config.get_minimize() {
            dfa.minimize();
        }
        if self.config.get_accelerate() {
            dfa.accelerate();
        }
        if !self.config.get_specialize_start_states() {
            dfa.special.set_no_special_start_states();
        }
        dfa.set_universal_starts();
        Ok(dfa)
    }
    pub fn configure(&mut self, config: Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {}
    #[cfg(feature = "syntax")]
    pub fn thompson(&mut self, config: thompson::Config) -> &mut Builder {}
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
