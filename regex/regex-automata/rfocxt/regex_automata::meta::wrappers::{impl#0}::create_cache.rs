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
pub(crate) struct PikeVM(PikeVMEngine);
#[derive(Clone, Debug)]
pub(crate) struct PikeVMCache(Option<pikevm::Cache>);
#[derive(Debug)]
pub(crate) struct PikeVMEngine(pikevm::PikeVM);
impl PikeVM {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<PikeVM, BuildError> {}
    pub(crate) fn create_cache(&self) -> PikeVMCache {
        PikeVMCache::new(self)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self) -> &PikeVMEngine {}
}
impl PikeVMCache {
    pub(crate) fn none() -> PikeVMCache {}
    pub(crate) fn new(builder: &PikeVM) -> PikeVMCache {
        PikeVMCache(Some(builder.get().0.create_cache()))
    }
    pub(crate) fn reset(&mut self, builder: &PikeVM) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
