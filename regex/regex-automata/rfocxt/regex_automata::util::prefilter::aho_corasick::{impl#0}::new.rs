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
impl AhoCorasick {
    pub(crate) fn new<B: AsRef<[u8]>>(
        kind: MatchKind,
        needles: &[B],
    ) -> Option<AhoCorasick> {
        #[cfg(not(feature = "perf-literal-multisubstring"))] { None }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            let ac_match_kind = match kind {
                MatchKind::LeftmostFirst | MatchKind::All => {
                    aho_corasick::MatchKind::LeftmostFirst
                }
            };
            let ac_kind = if needles.len() <= 500 {
                aho_corasick::AhoCorasickKind::DFA
            } else {
                aho_corasick::AhoCorasickKind::ContiguousNFA
            };
            let result = aho_corasick::AhoCorasick::builder()
                .kind(Some(ac_kind))
                .match_kind(ac_match_kind)
                .start_kind(aho_corasick::StartKind::Both)
                .prefilter(false)
                .build(needles);
            let ac = match result {
                Ok(ac) => ac,
                Err(_err) => {
                    debug!("aho-corasick prefilter failed to build: {}", _err);
                    return None;
                }
            };
            Some(AhoCorasick { ac })
        }
    }
}
