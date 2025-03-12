pub(crate) fn prefixes<H>(kind: MatchKind, hirs: &[H]) -> literal::Seq
where
    H: core::borrow::Borrow<Hir>,
{
    let mut extractor = literal::Extractor::new();
    extractor.kind(literal::ExtractKind::Prefix);

    let mut prefixes = literal::Seq::empty();
    for hir in hirs {
        prefixes.union(&mut extractor.extract(hir.borrow()));
    }
    debug!(
        "prefixes (len={:?}, exact={:?}) extracted before optimization: {:?}",
        prefixes.len(),
        prefixes.is_exact(),
        prefixes
    );
    match kind {
        MatchKind::All => {
            prefixes.sort();
            prefixes.dedup();
        }
        MatchKind::LeftmostFirst => {
            prefixes.optimize_for_prefix_by_preference();
        }
    }
    debug!(
        "prefixes (len={:?}, exact={:?}) extracted after optimization: {:?}",
        prefixes.len(),
        prefixes.is_exact(),
        prefixes
    );
    prefixes
}