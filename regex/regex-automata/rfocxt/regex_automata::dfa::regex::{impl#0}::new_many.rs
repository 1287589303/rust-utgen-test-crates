#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "dfa-build")]
use crate::dfa::dense::BuildError;
use crate::{
    dfa::{automaton::Automaton, dense},
    util::{iter, search::Input},
    Anchored, Match, MatchError,
};
#[cfg(feature = "alloc")]
use crate::{
    dfa::{sparse, StartKind},
    util::search::MatchKind,
};
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
#[derive(Clone, Debug)]
pub struct Builder {
    #[cfg(feature = "dfa-build")]
    dfa: dense::Builder,
}
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[cfg(all(feature = "syntax", feature = "dfa-build"))]
impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, BuildError> {}
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError> {
        Builder::new().build_many(patterns)
    }
}
impl Builder {
    pub fn new() -> Builder {
        Builder {
            #[cfg(feature = "dfa-build")]
            dfa: dense::Builder::new(),
        }
    }
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn build(&self, pattern: &str) -> Result<Regex, BuildError> {}
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn build_sparse(
        &self,
        pattern: &str,
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError> {}
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<Regex, BuildError> {
        let forward = self.dfa.build_many(patterns)?;
        let reverse = self
            .dfa
            .clone()
            .configure(
                dense::Config::new()
                    .prefilter(None)
                    .specialize_start_states(false)
                    .start_kind(StartKind::Anchored)
                    .match_kind(MatchKind::All),
            )
            .thompson(crate::nfa::thompson::Config::new().reverse(true))
            .build_many(patterns)?;
        Ok(self.build_from_dfas(forward, reverse))
    }
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn build_many_sparse<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError> {}
    pub fn build_from_dfas<A: Automaton>(&self, forward: A, reverse: A) -> Regex<A> {}
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder {}
    #[cfg(all(feature = "syntax", feature = "dfa-build"))]
    pub fn thompson(&mut self, config: crate::nfa::thompson::Config) -> &mut Builder {}
    #[cfg(feature = "dfa-build")]
    pub fn dense(&mut self, config: dense::Config) -> &mut Builder {}
}
