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
impl PrefilterI for AhoCorasick {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    fn memory_usage(&self) -> usize {}
    fn is_fast(&self) -> bool {
        #[cfg(not(feature = "perf-literal-multisubstring"))] { unreachable!() }
        #[cfg(feature = "perf-literal-multisubstring")] { false }
    }
}
