use core::{fmt::Debug, panic::{RefUnwindSafe, UnwindSafe}};
use alloc::sync::Arc;
use regex_syntax::hir::{literal, Hir};
use crate::{
    meta::{
        error::{BuildError, RetryError, RetryFailError, RetryQuadraticError},
        regex::{Cache, RegexInfo},
        reverse_inner, wrappers,
    },
    nfa::thompson::{self, WhichCaptures, NFA},
    util::{
        captures::{Captures, GroupInfo},
        look::LookMatcher, prefilter::{self, Prefilter, PrefilterI},
        primitives::{NonMaxUsize, PatternID},
        search::{Anchored, HalfMatch, Input, Match, MatchKind, PatternSet},
    },
};
pub(super) trait Strategy: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn group_info(&self) -> &GroupInfo;
    fn create_cache(&self) -> Cache;
    fn reset_cache(&self, cache: &mut Cache);
    fn is_accelerated(&self) -> bool;
    fn memory_usage(&self) -> usize;
    fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>;
    fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>;
    fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool;
    fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID>;
    fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    );
}
#[derive(Debug)]
struct ReverseSuffix {
    core: Core,
    pre: Prefilter,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HalfMatch {
    /// The pattern ID.
    pattern: PatternID,
    /// The offset of the match.
    ///
    /// For forward searches, the offset is exclusive. For reverse searches,
    /// the offset is inclusive.
    offset: usize,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Debug)]
struct Core {
    info: RegexInfo,
    pre: Option<Prefilter>,
    nfa: NFA,
    nfarev: Option<NFA>,
    pikevm: wrappers::PikeVM,
    backtrack: wrappers::BoundedBacktracker,
    onepass: wrappers::OnePass,
    hybrid: wrappers::Hybrid,
    dfa: wrappers::DFA,
}
#[derive(Debug)]
pub(crate) struct RetryFailError {
    offset: usize,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Debug)]
pub(crate) struct RetryQuadraticError(());
#[derive(Debug, Clone)]
pub struct Cache {
    pub(crate) capmatches: Captures,
    pub(crate) pikevm: wrappers::PikeVMCache,
    pub(crate) backtrack: wrappers::BoundedBacktrackerCache,
    pub(crate) onepass: wrappers::OnePassCache,
    pub(crate) hybrid: wrappers::HybridCache,
    pub(crate) revhybrid: wrappers::ReverseHybridCache,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Match {
    /// The pattern ID.
    pattern: PatternID,
    /// The underlying match span.
    span: Span,
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
#[derive(Debug)]
pub(crate) enum RetryError {
    Quadratic(RetryQuadraticError),
    Fail(RetryFailError),
}
impl Strategy for ReverseSuffix {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn group_info(&self) -> &GroupInfo {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn create_cache(&self) -> Cache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn reset_cache(&self, cache: &mut Cache) {}
    fn is_accelerated(&self) -> bool {}
    fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match> {
        if input.get_anchored().is_anchored() {
            return self.core.search(cache, input);
        }
        match self.try_search_half_start(cache, input) {
            Err(RetryError::Quadratic(_err)) => {
                trace!("reverse suffix optimization failed: {}", _err);
                self.core.search(cache, input)
            }
            Err(RetryError::Fail(_err)) => {
                trace!("reverse suffix reverse fast search failed: {}", _err);
                self.core.search_nofail(cache, input)
            }
            Ok(None) => None,
            Ok(Some(hm_start)) => {
                let fwdinput = input
                    .clone()
                    .anchored(Anchored::Pattern(hm_start.pattern()))
                    .span(hm_start.offset()..input.end());
                match self.try_search_half_fwd(cache, &fwdinput) {
                    Err(_err) => {
                        trace!("reverse suffix forward fast search failed: {}", _err);
                        self.core.search_nofail(cache, input)
                    }
                    Ok(None) => {
                        unreachable!(
                            "suffix match plus reverse match implies \
						     there must be a match",
                        )
                    }
                    Ok(Some(hm_end)) => {
                        Some(
                            Match::new(
                                hm_start.pattern(),
                                hm_start.offset()..hm_end.offset(),
                            ),
                        )
                    }
                }
            }
        }
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {}
}
impl Anchored {
    #[inline]
    pub fn is_anchored(&self) -> bool {
        matches!(* self, Anchored::Yes | Anchored::Pattern(_))
    }
    #[inline]
    pub fn pattern(&self) -> Option<PatternID> {}
}
impl ReverseSuffix {
    fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseSuffix, Core> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_half_start(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryError> {
        let mut span = input.get_span();
        let mut min_start = 0;
        loop {
            let litmatch = match self.pre.find(input.haystack(), span) {
                None => return Ok(None),
                Some(span) => span,
            };
            trace!("reverse suffix scan found suffix match at {:?}", litmatch);
            let revinput = input
                .clone()
                .anchored(Anchored::Yes)
                .span(input.start()..litmatch.end);
            match self.try_search_half_rev_limited(cache, &revinput, min_start)? {
                None => {
                    if span.start >= span.end {
                        break;
                    }
                    span.start = litmatch.start.checked_add(1).unwrap();
                }
                Some(hm) => return Ok(Some(hm)),
            }
            min_start = litmatch.end;
        }
        Ok(None)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_half_fwd(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryFailError> {
        if let Some(e) = self.core.dfa.get(&input) {
            trace!(
                "using full DFA for forward reverse suffix search at {:?}", input
                .get_span()
            );
            e.try_search_half_fwd(&input)
        } else if let Some(e) = self.core.hybrid.get(&input) {
            trace!(
                "using lazy DFA for forward reverse suffix search at {:?}", input
                .get_span()
            );
            e.try_search_half_fwd(&mut cache.hybrid, &input)
        } else {
            unreachable!("ReverseSuffix always has a DFA")
        }
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_half_rev_limited(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {}
}
impl HalfMatch {
    #[inline]
    pub fn new(pattern: PatternID, offset: usize) -> HalfMatch {}
    #[inline]
    pub fn must(pattern: usize, offset: usize) -> HalfMatch {}
    #[inline]
    pub fn pattern(&self) -> PatternID {
        self.pattern
    }
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }
}
impl<'h> Input<'h> {
    #[inline]
    pub fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h> {}
    #[inline]
    pub fn span<S: Into<Span>>(mut self, span: S) -> Input<'h> {}
    #[inline]
    pub fn range<R: RangeBounds<usize>>(mut self, range: R) -> Input<'h> {}
    #[inline]
    pub fn anchored(mut self, mode: Anchored) -> Input<'h> {
        self.set_anchored(mode);
        self
    }
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
    pub fn end(&self) -> usize {
        self.get_span().end
    }
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
impl Core {
    fn new(
        info: RegexInfo,
        pre: Option<Prefilter>,
        hirs: &[&Hir],
    ) -> Result<Core, BuildError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_mayfail(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<Result<Option<Match>, RetryFailError>> {}
    fn search_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match> {
        let caps = &mut cache.capmatches;
        caps.set_pattern(None);
        let pid = if let Some(ref e) = self.onepass.get(input) {
            trace!("using OnePass for search at {:?}", input.get_span());
            e.search_slots(&mut cache.onepass, input, caps.slots_mut())
        } else if let Some(ref e) = self.backtrack.get(input) {
            trace!("using BoundedBacktracker for search at {:?}", input.get_span());
            e.search_slots(&mut cache.backtrack, input, caps.slots_mut())
        } else {
            trace!("using PikeVM for search at {:?}", input.get_span());
            let e = self.pikevm.get();
            e.search_slots(&mut cache.pikevm, input, caps.slots_mut())
        };
        caps.set_pattern(pid);
        caps.get_match()
    }
    fn search_half_nofail(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<HalfMatch> {}
    fn search_slots_nofail(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    fn is_match_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> bool {}
    fn is_capture_search_needed(&self, slots_len: usize) -> bool {}
}
