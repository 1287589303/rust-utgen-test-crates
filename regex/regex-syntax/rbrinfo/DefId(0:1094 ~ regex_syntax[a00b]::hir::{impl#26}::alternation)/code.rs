fn alternation(alts: &[Hir]) -> Properties {
        Properties::union(alts.iter().map(|hir| hir.properties()))
    }