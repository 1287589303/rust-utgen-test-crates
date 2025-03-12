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
#[cfg(feature = "syntax")]
pub(crate) fn suffixes<H>(kind: MatchKind, hirs: &[H]) -> literal::Seq
where
    H: core::borrow::Borrow<Hir>,
{
    let mut extractor = literal::Extractor::new();
    extractor.kind(literal::ExtractKind::Suffix);
    let mut suffixes = literal::Seq::empty();
    for hir in hirs {
        suffixes.union(&mut extractor.extract(hir.borrow()));
    }
    debug!(
        "suffixes (len={:?}, exact={:?}) extracted before optimization: {:?}", suffixes
        .len(), suffixes.is_exact(), suffixes
    );
    match kind {
        MatchKind::All => {
            suffixes.sort();
            suffixes.dedup();
        }
        MatchKind::LeftmostFirst => {
            suffixes.optimize_for_suffix_by_preference();
        }
    }
    debug!(
        "suffixes (len={:?}, exact={:?}) extracted after optimization: {:?}", suffixes
        .len(), suffixes.is_exact(), suffixes
    );
    suffixes
}
