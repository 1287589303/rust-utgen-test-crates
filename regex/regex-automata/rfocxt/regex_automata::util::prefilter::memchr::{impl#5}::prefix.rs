use crate::util::{prefilter::PrefilterI, search::{MatchKind, Span}};
pub(crate) trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn memory_usage(&self) -> usize;
    fn is_fast(&self) -> bool;
}
#[derive(Clone, Debug)]
pub(crate) struct Memchr3(u8, u8, u8);
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    /// The start offset of the span, inclusive.
    pub start: usize,
    /// The end offset of the span, exclusive.
    pub end: usize,
}
impl PrefilterI for Memchr3 {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {
        let b = *haystack.get(span.start)?;
        if self.0 == b || self.1 == b || self.2 == b {
            Some(Span {
                start: span.start,
                end: span.start + 1,
            })
        } else {
            None
        }
    }
    fn memory_usage(&self) -> usize {}
    fn is_fast(&self) -> bool {}
}
