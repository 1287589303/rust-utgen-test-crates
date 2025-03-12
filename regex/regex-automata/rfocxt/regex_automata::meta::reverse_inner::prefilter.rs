use alloc::vec::Vec;
use regex_syntax::hir::{self, literal, Hir, HirKind};
use crate::{util::prefilter::Prefilter, MatchKind};
#[derive(Clone, Debug)]
pub struct Prefilter {
    #[cfg(not(feature = "alloc"))]
    _unused: (),
    #[cfg(feature = "alloc")]
    pre: Arc<dyn PrefilterI>,
    #[cfg(feature = "alloc")]
    is_fast: bool,
    #[cfg(feature = "alloc")]
    max_needle_len: usize,
}
fn prefilter(hir: &Hir) -> Option<Prefilter> {
    let mut extractor = literal::Extractor::new();
    extractor.kind(literal::ExtractKind::Prefix);
    let mut prefixes = extractor.extract(hir);
    debug!(
        "inner prefixes (len={:?}) extracted before optimization: {:?}", prefixes.len(),
        prefixes
    );
    prefixes.make_inexact();
    prefixes.optimize_for_prefix_by_preference();
    debug!(
        "inner prefixes (len={:?}) extracted after optimization: {:?}", prefixes.len(),
        prefixes
    );
    prefixes.literals().and_then(|lits| Prefilter::new(MatchKind::LeftmostFirst, lits))
}
