use core::{borrow::Borrow, fmt::Debug, panic::{RefUnwindSafe, UnwindSafe}};
#[cfg(feature = "alloc")]
use alloc::sync::Arc;
#[cfg(feature = "syntax")]
use regex_syntax::hir::{literal, Hir};
use crate::util::search::{MatchKind, Span};
pub(crate) use crate::util::prefilter::{
    aho_corasick::AhoCorasick, byteset::ByteSet, memchr::{Memchr, Memchr2, Memchr3},
    memmem::Memmem, teddy::Teddy,
};
pub(crate) trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn memory_usage(&self) -> usize;
    fn is_fast(&self) -> bool;
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
#[derive(Clone, Debug)]
pub(crate) enum Choice {
    Memchr(Memchr),
    Memchr2(Memchr2),
    Memchr3(Memchr3),
    Memmem(Memmem),
    Teddy(Teddy),
    ByteSet(ByteSet),
    AhoCorasick(AhoCorasick),
}
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchKind {
    /// Report all possible matches.
    All,
    /// Report only the leftmost matches. When multiple leftmost matches exist,
    /// report the match corresponding to the part of the regex that appears
    /// first in the syntax.
    LeftmostFirst,
}
impl Prefilter {
    pub fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Prefilter> {
        Choice::new(kind, needles)
            .and_then(|choice| {
                let max_needle_len = needles
                    .iter()
                    .map(|b| b.as_ref().len())
                    .max()
                    .unwrap_or(0);
                Prefilter::from_choice(choice, max_needle_len)
            })
    }
    fn from_choice(choice: Choice, max_needle_len: usize) -> Option<Prefilter> {}
    #[cfg(feature = "syntax")]
    pub fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter> {}
    #[cfg(feature = "syntax")]
    pub fn from_hirs_prefix<H: Borrow<Hir>>(
        kind: MatchKind,
        hirs: &[H],
    ) -> Option<Prefilter> {}
    #[inline]
    pub fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    #[inline]
    pub fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
    #[inline]
    pub fn max_needle_len(&self) -> usize {}
    #[inline]
    pub fn is_fast(&self) -> bool {}
}
impl Choice {
    pub(crate) fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Choice> {
        if needles.len() == 0 {
            debug!("prefilter building failed: found empty set of literals");
            return None;
        }
        if needles.iter().any(|n| n.as_ref().is_empty()) {
            debug!("prefilter building failed: literals match empty string");
            return None;
        }
        if let Some(pre) = Memchr::new(kind, needles) {
            debug!("prefilter built: memchr");
            return Some(Choice::Memchr(pre));
        }
        if let Some(pre) = Memchr2::new(kind, needles) {
            debug!("prefilter built: memchr2");
            return Some(Choice::Memchr2(pre));
        }
        if let Some(pre) = Memchr3::new(kind, needles) {
            debug!("prefilter built: memchr3");
            return Some(Choice::Memchr3(pre));
        }
        if let Some(pre) = Memmem::new(kind, needles) {
            debug!("prefilter built: memmem");
            return Some(Choice::Memmem(pre));
        }
        if let Some(pre) = Teddy::new(kind, needles) {
            debug!("prefilter built: teddy");
            return Some(Choice::Teddy(pre));
        }
        if let Some(pre) = ByteSet::new(kind, needles) {
            debug!("prefilter built: byteset");
            return Some(Choice::ByteSet(pre));
        }
        if let Some(pre) = AhoCorasick::new(kind, needles) {
            debug!("prefilter built: aho-corasick");
            return Some(Choice::AhoCorasick(pre));
        }
        debug!("prefilter building failed: no strategy could be found");
        None
    }
}
