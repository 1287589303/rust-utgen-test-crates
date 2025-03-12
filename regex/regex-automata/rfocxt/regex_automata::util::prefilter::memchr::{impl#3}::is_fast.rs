use crate::util::{prefilter::PrefilterI, search::{MatchKind, Span}};
pub(crate) trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn memory_usage(&self) -> usize;
    fn is_fast(&self) -> bool;
}
#[derive(Clone, Debug)]
pub(crate) struct Memchr2(u8, u8);
impl PrefilterI for Memchr2 {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    fn memory_usage(&self) -> usize {}
    fn is_fast(&self) -> bool {
        true
    }
}
