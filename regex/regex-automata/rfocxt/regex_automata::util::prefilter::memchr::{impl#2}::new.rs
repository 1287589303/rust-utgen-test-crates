use crate::util::{prefilter::PrefilterI, search::{MatchKind, Span}};
pub(crate) trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn memory_usage(&self) -> usize;
    fn is_fast(&self) -> bool;
}
#[derive(Clone, Debug)]
pub(crate) struct Memchr2(u8, u8);
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
impl Memchr2 {
    pub(crate) fn new<B: AsRef<[u8]>>(
        _kind: MatchKind,
        needles: &[B],
    ) -> Option<Memchr2> {
        #[cfg(not(feature = "perf-literal-substring"))] { None }
        #[cfg(feature = "perf-literal-substring")]
        {
            if needles.len() != 2 {
                return None;
            }
            if !needles.iter().all(|n| n.as_ref().len() == 1) {
                return None;
            }
            let b1 = needles[0].as_ref()[0];
            let b2 = needles[1].as_ref()[0];
            Some(Memchr2(b1, b2))
        }
    }
}
