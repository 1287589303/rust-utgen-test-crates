use crate::util::{prefilter::PrefilterI, search::{MatchKind, Span}};
pub(crate) trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn memory_usage(&self) -> usize;
    fn is_fast(&self) -> bool;
}
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
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
impl ByteSet {
    pub(crate) fn new<B: AsRef<[u8]>>(
        _kind: MatchKind,
        needles: &[B],
    ) -> Option<ByteSet> {
        #[cfg(not(feature = "perf-literal-multisubstring"))] { None }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            let mut set = [false; 256];
            for needle in needles.iter() {
                let needle = needle.as_ref();
                if needle.len() != 1 {
                    return None;
                }
                set[usize::from(needle[0])] = true;
            }
            Some(ByteSet(set))
        }
    }
}
