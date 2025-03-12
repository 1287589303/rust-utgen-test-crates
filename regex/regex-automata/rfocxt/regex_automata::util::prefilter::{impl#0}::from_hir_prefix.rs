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
    pub fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Prefilter> {}
    fn from_choice(choice: Choice, max_needle_len: usize) -> Option<Prefilter> {}
    #[cfg(feature = "syntax")]
    pub fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter> {
        Prefilter::from_hirs_prefix(kind, &[hir])
    }
    #[cfg(feature = "syntax")]
    pub fn from_hirs_prefix<H: Borrow<Hir>>(
        kind: MatchKind,
        hirs: &[H],
    ) -> Option<Prefilter> {
        prefixes(kind, hirs).literals().and_then(|lits| Prefilter::new(kind, lits))
    }
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
