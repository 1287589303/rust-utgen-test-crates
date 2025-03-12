use crate::util::{prefilter::PrefilterI, search::{MatchKind, Span}};
pub(crate) trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn memory_usage(&self) -> usize;
    fn is_fast(&self) -> bool;
}
#[derive(Clone, Debug)]
pub(crate) struct Memmem {
    #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
    _unused: (),
    #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
    finder: memchr::memmem::Finder<'static>,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    /// The start offset of the span, inclusive.
    pub start: usize,
    /// The end offset of the span, exclusive.
    pub end: usize,
}
impl PrefilterI for Memmem {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
        { unreachable!() }
        #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
        {
            let needle = self.finder.needle();
            if haystack[span].starts_with(needle) {
                Some(Span {
                    end: span.start + needle.len(),
                    ..span
                })
            } else {
                None
            }
        }
    }
    fn memory_usage(&self) -> usize {}
    fn is_fast(&self) -> bool {}
}
