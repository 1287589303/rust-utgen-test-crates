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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Match {
    /// The pattern ID.
    pattern: PatternID,
    /// The underlying match span.
    span: Span,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonMaxUsize(NonZeroUsize);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
impl Match {
    #[inline]
    pub fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match {}
    #[inline]
    pub fn must<S: Into<Span>>(pattern: usize, span: S) -> Match {}
    #[inline]
    pub fn pattern(&self) -> PatternID {
        self.pattern
    }
    #[inline]
    pub fn start(&self) -> usize {
        self.span().start
    }
    #[inline]
    pub fn end(&self) -> usize {
        self.span().end
    }
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {}
    #[inline]
    pub fn span(&self) -> Span {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
}
impl NonMaxUsize {
    #[inline]
    pub fn new(value: usize) -> Option<NonMaxUsize> {
        NonZeroUsize::new(value.wrapping_add(1)).map(NonMaxUsize)
    }
    #[inline]
    pub fn get(self) -> usize {}
}
#[cfg_attr(feature = "perf-inline", inline(always))]
fn copy_match_to_slots(m: Match, slots: &mut [Option<NonMaxUsize>]) {
    let slot_start = m.pattern().as_usize() * 2;
    let slot_end = slot_start + 1;
    if let Some(slot) = slots.get_mut(slot_start) {
        *slot = NonMaxUsize::new(m.start());
    }
    if let Some(slot) = slots.get_mut(slot_end) {
        *slot = NonMaxUsize::new(m.end());
    }
}
