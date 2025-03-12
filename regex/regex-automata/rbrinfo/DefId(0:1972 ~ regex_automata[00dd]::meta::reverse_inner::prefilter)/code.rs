fn prefilter(hir: &Hir) -> Option<Prefilter> {
    let mut extractor = literal::Extractor::new();
    extractor.kind(literal::ExtractKind::Prefix);
    let mut prefixes = extractor.extract(hir);
    debug!(
        "inner prefixes (len={:?}) extracted before optimization: {:?}",
        prefixes.len(),
        prefixes
    );
    // Since these are inner literals, we know they cannot be exact. But the
    // extractor doesn't know this. We mark them as inexact because this might
    // impact literal optimization. Namely, optimization weights "all literals
    // are exact" as very high, because it presumes that any match results in
    // an overall match. But of course, that is not the case here.
    //
    // In practice, this avoids plucking out a ASCII-only \s as an alternation
    // of single-byte whitespace characters.
    prefixes.make_inexact();
    prefixes.optimize_for_prefix_by_preference();
    debug!(
        "inner prefixes (len={:?}) extracted after optimization: {:?}",
        prefixes.len(),
        prefixes
    );
    prefixes
        .literals()
        .and_then(|lits| Prefilter::new(MatchKind::LeftmostFirst, lits))
}