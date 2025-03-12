pub fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter> {
        Prefilter::from_hirs_prefix(kind, &[hir])
    }