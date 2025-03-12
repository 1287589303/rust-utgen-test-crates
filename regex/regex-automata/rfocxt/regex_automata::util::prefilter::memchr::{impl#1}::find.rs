use crate::util::{prefilter::PrefilterI, search::{MatchKind, Span}};
pub(crate) trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn memory_usage(&self) -> usize;
    fn is_fast(&self) -> bool;
}
#[derive(Clone, Debug)]
pub(crate) struct Memchr(u8);
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    /// The start offset of the span, inclusive.
    pub start: usize,
    /// The end offset of the span, exclusive.
    pub end: usize,
}
impl PrefilterI for Memchr {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(feature = "perf-literal-substring"))] { unreachable!() }
        #[cfg(feature = "perf-literal-substring")]
        {
            memchr::memchr(self.0, &haystack[span])
                .map(|i| {
                    let start = span.start + i;
                    let end = start + 1;
                    Span { start, end }
                })
        }
    }
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    fn memory_usage(&self) -> usize {}
    fn is_fast(&self) -> bool {}
}
