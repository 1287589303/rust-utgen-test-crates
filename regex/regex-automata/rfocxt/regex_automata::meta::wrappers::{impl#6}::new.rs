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
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug)]
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
impl OnePass {
    pub(crate) fn new(info: &RegexInfo, nfa: &NFA) -> OnePass {
        OnePass(OnePassEngine::new(info, nfa))
    }
    pub(crate) fn create_cache(&self) -> OnePassCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, input: &Input<'_>) -> Option<&OnePassEngine> {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl OnePassEngine {
    pub(crate) fn new(info: &RegexInfo, nfa: &NFA) -> Option<OnePassEngine> {
        #[cfg(feature = "dfa-onepass")]
        {
            if !info.config().get_onepass() {
                return None;
            }
            if info.props_union().explicit_captures_len() == 0
                && !info.props_union().look_set().contains_word_unicode()
            {
                debug!("not building OnePass because it isn't worth it");
                return None;
            }
            let onepass_config = onepass::Config::new()
                .match_kind(info.config().get_match_kind())
                .starts_for_each_pattern(true)
                .byte_classes(info.config().get_byte_classes())
                .size_limit(info.config().get_onepass_size_limit());
            let result = onepass::Builder::new()
                .configure(onepass_config)
                .build_from_nfa(nfa.clone());
            let engine = match result {
                Ok(engine) => engine,
                Err(_err) => {
                    debug!("OnePass failed to build: {}", _err);
                    return None;
                }
            };
            debug!("OnePass built, {} bytes", engine.memory_usage());
            Some(OnePassEngine(engine))
        }
        #[cfg(not(feature = "dfa-onepass"))] { None }
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn search_slots(
        &self,
        cache: &mut OnePassCache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    pub(crate) fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn get_nfa(&self) -> &NFA {}
}
