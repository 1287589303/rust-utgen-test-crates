pub fn from_hirs_prefix<H: Borrow<Hir>>(
        kind: MatchKind,
        hirs: &[H],
    ) -> Option<Prefilter> {
        prefixes(kind, hirs)
            .literals()
            .and_then(|lits| Prefilter::new(kind, lits))
    }