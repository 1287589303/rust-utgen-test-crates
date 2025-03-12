pub(crate) fn new<B: AsRef<[u8]>>(
        kind: MatchKind,
        needles: &[B],
    ) -> Option<Teddy> {
        #[cfg(not(feature = "perf-literal-multisubstring"))]
        {
            None
        }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            // We only really support leftmost-first semantics. In
            // theory we could at least support leftmost-longest, as the
            // aho-corasick crate does, but regex-automata doesn't know about
            // leftmost-longest currently.
            //
            // And like the aho-corasick prefilter, if we're using `All`
            // semantics, then we can still use leftmost semantics for a
            // prefilter. (This might be a suspicious choice for the literal
            // engine, which uses a prefilter as a regex engine directly, but
            // that only happens when using leftmost-first semantics.)
            let (packed_match_kind, ac_match_kind) = match kind {
                MatchKind::LeftmostFirst | MatchKind::All => (
                    aho_corasick::packed::MatchKind::LeftmostFirst,
                    aho_corasick::MatchKind::LeftmostFirst,
                ),
            };
            let minimum_len =
                needles.iter().map(|n| n.as_ref().len()).min().unwrap_or(0);
            let packed = aho_corasick::packed::Config::new()
                .match_kind(packed_match_kind)
                .builder()
                .extend(needles)
                .build()?;
            let anchored_ac = aho_corasick::dfa::DFA::builder()
                .match_kind(ac_match_kind)
                .start_kind(aho_corasick::StartKind::Anchored)
                .prefilter(false)
                .build(needles)
                .ok()?;
            Some(Teddy { searcher: packed, anchored_ac, minimum_len })
        }
    }