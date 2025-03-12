use alloc::vec::Vec;
use crate::{
    meta::{
        error::{BuildError, RetryError, RetryFailError},
        regex::RegexInfo,
    },
    nfa::thompson::{pikevm, NFA},
    util::{prefilter::Prefilter, primitives::NonMaxUsize},
    HalfMatch, Input, Match, MatchKind, PatternID, PatternSet,
};
#[cfg(feature = "dfa-build")]
use crate::dfa;
#[cfg(feature = "dfa-onepass")]
use crate::dfa::onepass;
#[cfg(feature = "hybrid")]
use crate::hybrid;
#[cfg(feature = "nfa-backtrack")]
use crate::nfa::thompson::backtrack;
#[derive(Debug)]
pub(crate) struct ReverseHybrid(Option<ReverseHybridEngine>);
#[derive(Debug)]
pub(crate) struct ReverseHybridEngine(
    #[cfg(feature = "hybrid")]
    hybrid::dfa::DFA,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
impl ReverseHybrid {
    pub(crate) fn none() -> ReverseHybrid {}
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseHybrid {}
    pub(crate) fn create_cache(&self) -> ReverseHybridCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseHybridEngine> {
        let engine = self.0.as_ref()?;
        Some(engine)
    }
}
