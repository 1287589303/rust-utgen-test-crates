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
#[cfg(feature = "alloc")]
impl<P: PrefilterI + ?Sized> PrefilterI for Arc<P> {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_fast(&self) -> bool {
        (&**self).is_fast()
    }
}
