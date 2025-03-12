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
pub(crate) struct Memchr3(u8, u8, u8);
#[derive(Clone, Debug)]
pub(crate) struct Memchr2(u8, u8);
#[derive(Clone, Debug)]
pub(crate) struct AhoCorasick {
    #[cfg(not(feature = "perf-literal-multisubstring"))]
    _unused: (),
    #[cfg(feature = "perf-literal-multisubstring")]
    ac: aho_corasick::AhoCorasick,
}
#[derive(Clone, Debug)]
pub(crate) struct Teddy {
    #[cfg(not(feature = "perf-literal-multisubstring"))]
    _unused: (),
    /// The actual Teddy searcher.
    ///
    /// Technically, it's possible that Teddy doesn't actually get used, since
    /// Teddy does require its haystack to at least be of a certain size
    /// (usually around the size of whatever vector is being used, so ~16
    /// or ~32 bytes). For haystacks shorter than that, the implementation
    /// currently uses Rabin-Karp.
    #[cfg(feature = "perf-literal-multisubstring")]
    searcher: aho_corasick::packed::Searcher,
    /// When running an anchored search, the packed searcher can't handle it so
    /// we defer to Aho-Corasick itself. Kind of sad, but changing the packed
    /// searchers to support anchored search would be difficult at worst and
    /// annoying at best. Since packed searchers only apply to small numbers of
    /// literals, we content ourselves that this is not much of an added cost.
    /// (That packed searchers only work with a small number of literals is
    /// also why we use a DFA here. Otherwise, the memory usage of a DFA would
    /// likely be unacceptable.)
    #[cfg(feature = "perf-literal-multisubstring")]
    anchored_ac: aho_corasick::dfa::DFA,
    /// The length of the smallest literal we look for.
    ///
    /// We use this as a heuristic to figure out whether this will be "fast" or
    /// not. Generally, the longer the better, because longer needles are more
    /// discriminating and thus reduce false positive rate.
    #[cfg(feature = "perf-literal-multisubstring")]
    minimum_len: usize,
}
#[derive(Clone, Debug)]
pub(crate) struct Memchr(u8);
#[derive(Clone, Debug)]
pub(crate) struct Memmem {
    #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
    _unused: (),
    #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
    finder: memchr::memmem::Finder<'static>,
}
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
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
impl Prefilter {
    pub fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Prefilter> {}
    fn from_choice(choice: Choice, max_needle_len: usize) -> Option<Prefilter> {
        #[cfg(not(feature = "alloc"))] { None }
        #[cfg(feature = "alloc")]
        {
            let pre: Arc<dyn PrefilterI> = match choice {
                Choice::Memchr(p) => Arc::new(p),
                Choice::Memchr2(p) => Arc::new(p),
                Choice::Memchr3(p) => Arc::new(p),
                Choice::Memmem(p) => Arc::new(p),
                Choice::Teddy(p) => Arc::new(p),
                Choice::ByteSet(p) => Arc::new(p),
                Choice::AhoCorasick(p) => Arc::new(p),
            };
            let is_fast = pre.is_fast();
            Some(Prefilter {
                pre,
                is_fast,
                max_needle_len,
            })
        }
    }
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
