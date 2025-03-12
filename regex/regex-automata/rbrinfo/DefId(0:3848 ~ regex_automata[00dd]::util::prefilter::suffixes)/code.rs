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
        "suffixes (len={:?}, exact={:?}) extracted before optimization: {:?}",
        suffixes.len(),
        suffixes.is_exact(),
        suffixes
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
        "suffixes (len={:?}, exact={:?}) extracted after optimization: {:?}",
        suffixes.len(),
        suffixes.is_exact(),
        suffixes
    );
    suffixes
}