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
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Anchored {
    /// Run an unanchored search. This means a match may occur anywhere at or
    /// after the start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    No,
    /// Run an anchored search. This means that a match must begin at the
    /// start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    Yes,
    /// Run an anchored search for a specific pattern. This means that a match
    /// must be for the given pattern and must begin at the start position of
    /// the search.
    Pattern(PatternID),
}
impl OnePass {
    pub(crate) fn new(info: &RegexInfo, nfa: &NFA) -> OnePass {}
    pub(crate) fn create_cache(&self) -> OnePassCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, input: &Input<'_>) -> Option<&OnePassEngine> {
        let engine = self.0.as_ref()?;
        if !input.get_anchored().is_anchored()
            && !engine.get_nfa().is_always_start_anchored()
        {
            return None;
        }
        Some(engine)
    }
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl OnePassEngine {
    pub(crate) fn new(info: &RegexInfo, nfa: &NFA) -> Option<OnePassEngine> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn search_slots(
        &self,
        cache: &mut OnePassCache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    pub(crate) fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn get_nfa(&self) -> &NFA {
        #[cfg(feature = "dfa-onepass")] { self.0.get_nfa() }
        #[cfg(not(feature = "dfa-onepass"))] { unreachable!() }
    }
}
impl Anchored {
    #[inline]
    pub fn is_anchored(&self) -> bool {
        matches!(* self, Anchored::Yes | Anchored::Pattern(_))
    }
    #[inline]
    pub fn pattern(&self) -> Option<PatternID> {}
}
impl<'h> Input<'h> {
    #[inline]
    pub fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h> {}
    #[inline]
    pub fn span<S: Into<Span>>(mut self, span: S) -> Input<'h> {}
    #[inline]
    pub fn range<R: RangeBounds<usize>>(mut self, range: R) -> Input<'h> {}
    #[inline]
    pub fn anchored(mut self, mode: Anchored) -> Input<'h> {}
    #[inline]
    pub fn earliest(mut self, yes: bool) -> Input<'h> {}
    #[inline]
    pub fn set_span<S: Into<Span>>(&mut self, span: S) {}
    #[inline]
    pub fn set_range<R: RangeBounds<usize>>(&mut self, range: R) {}
    #[inline]
    pub fn set_start(&mut self, start: usize) {}
    #[inline]
    pub fn set_end(&mut self, end: usize) {}
    #[inline]
    pub fn set_anchored(&mut self, mode: Anchored) {}
    #[inline]
    pub fn set_earliest(&mut self, yes: bool) {}
    #[inline]
    pub fn haystack(&self) -> &[u8] {}
    #[inline]
    pub fn start(&self) -> usize {}
    #[inline]
    pub fn end(&self) -> usize {}
    #[inline]
    pub fn get_span(&self) -> Span {}
    #[inline]
    pub fn get_range(&self) -> Range<usize> {}
    #[inline]
    pub fn get_anchored(&self) -> Anchored {
        self.anchored
    }
    #[inline]
    pub fn get_earliest(&self) -> bool {}
    #[inline]
    pub fn is_done(&self) -> bool {}
    #[inline]
    pub fn is_char_boundary(&self, offset: usize) -> bool {}
}
impl NFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<NFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError> {}
    pub fn always_match() -> NFA {}
    pub fn never_match() -> NFA {}
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
    pub fn is_always_start_anchored(&self) -> bool {
        self.start_anchored() == self.start_unanchored()
    }
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {}
    #[inline]
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
