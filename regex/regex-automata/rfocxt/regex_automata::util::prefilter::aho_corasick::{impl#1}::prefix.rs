use crate::util::{prefilter::PrefilterI, search::{MatchKind, Span}};
pub(crate) trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn memory_usage(&self) -> usize;
    fn is_fast(&self) -> bool;
}
#[derive(Clone, Debug)]
pub(crate) struct AhoCorasick {
    #[cfg(not(feature = "perf-literal-multisubstring"))]
    _unused: (),
    #[cfg(feature = "perf-literal-multisubstring")]
    ac: aho_corasick::AhoCorasick,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    /// The start offset of the span, inclusive.
    pub start: usize,
    /// The end offset of the span, exclusive.
    pub end: usize,
}
impl PrefilterI for AhoCorasick {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(feature = "perf-literal-multisubstring"))] { unreachable!() }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            let input = aho_corasick::Input::new(haystack)
                .anchored(aho_corasick::Anchored::Yes)
                .span(span.start..span.end);
            self.ac
                .find(input)
                .map(|m| Span {
                    start: m.start(),
                    end: m.end(),
                })
        }
    }
    fn memory_usage(&self) -> usize {}
    fn is_fast(&self) -> bool {}
}
