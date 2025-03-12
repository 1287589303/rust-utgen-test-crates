fn from_alternation_literals(
        info: &RegexInfo,
        hirs: &[&Hir],
    ) -> Option<Arc<dyn Strategy>> {
        use crate::util::prefilter::AhoCorasick;

        let lits = crate::meta::literal::alternation_literals(info, hirs)?;
        let ac = AhoCorasick::new(MatchKind::LeftmostFirst, &lits)?;
        Some(Pre::new(ac))
    }