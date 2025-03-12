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
pub(crate) struct ReverseDFA(Option<ReverseDFAEngine>);
#[derive(Debug)]
pub(crate) struct ReverseDFAEngine(
    #[cfg(feature = "dfa-build")]
    dfa::dense::DFA<Vec<u32>>,
    #[cfg(not(feature = "dfa-build"))]
    (),
);
impl ReverseDFA {
    pub(crate) fn none() -> ReverseDFA {}
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseDFA {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseDFAEngine> {}
    pub(crate) fn is_some(&self) -> bool {}
    pub(crate) fn memory_usage(&self) -> usize {
        self.0.as_ref().map_or(0, |e| e.memory_usage())
    }
}
