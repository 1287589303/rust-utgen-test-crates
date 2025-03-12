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
pub(crate) struct OnePass(Option<OnePassEngine>);
#[derive(Debug)]
pub(crate) struct OnePassEngine(
    #[cfg(feature = "dfa-onepass")]
    onepass::DFA,
    #[cfg(not(feature = "dfa-onepass"))]
    (),
);
impl OnePass {
    pub(crate) fn new(info: &RegexInfo, nfa: &NFA) -> OnePass {}
    pub(crate) fn create_cache(&self) -> OnePassCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, input: &Input<'_>) -> Option<&OnePassEngine> {}
    pub(crate) fn memory_usage(&self) -> usize {
        self.0.as_ref().map_or(0, |e| e.memory_usage())
    }
}
