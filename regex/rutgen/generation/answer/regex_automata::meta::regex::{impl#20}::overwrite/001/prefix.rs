// Answer 0

#[test]
fn test_overwrite_all_fields() {
    let original_config = Config::new()
        .match_kind(MatchKind::All)
        .utf8_empty(true)
        .auto_prefilter(true)
        .prefilter(Some(Prefilter {
            pre: Arc::new(/* implement PrefilterI */),
            is_fast: true,
            max_needle_len: 10,
        }))
        .which_captures(WhichCaptures::All)
        .nfa_size_limit(Some(100))
        .onepass_size_limit(Some(50))
        .hybrid_cache_capacity(20)
        .hybrid(true)
        .dfa(true)
        .dfa_size_limit(Some(200))
        .dfa_state_limit(Some(150))
        .onepass(true)
        .backtrack(true)
        .byte_classes(true)
        .line_terminator(42);

    let new_config = Config::new()
        .match_kind(MatchKind::LeftmostFirst)
        .utf8_empty(false)
        .auto_prefilter(false)
        .prefilter(None)
        .which_captures(WhichCaptures::Implicit)
        .nfa_size_limit(None)
        .onepass_size_limit(None)
        .hybrid_cache_capacity(30)
        .hybrid(false)
        .dfa(false)
        .dfa_size_limit(None)
        .dfa_state_limit(None)
        .onepass(false)
        .backtrack(false)
        .byte_classes(false)
        .line_terminator(255);

    original_config.overwrite(new_config);
}

#[test]
fn test_overwrite_default_fields() {
    let original_config = Config::default();

    let new_config = Config::new()
        .match_kind(MatchKind::All)
        .utf8_empty(true)
        .auto_prefilter(false)
        .prefilter(None)
        .which_captures(WhichCaptures::None)
        .nfa_size_limit(Some(200))
        .onepass_size_limit(None)
        .hybrid_cache_capacity(10)
        .hybrid(false)
        .dfa(true)
        .dfa_size_limit(None)
        .dfa_state_limit(Some(100))
        .onepass(true)
        .backtrack(false)
        .byte_classes(true)
        .line_terminator(1);

    original_config.overwrite(new_config);
}

#[test]
fn test_overwrite_edge_cases() {
    let original_config = Config::new()
        .match_kind(MatchKind::None)
        .utf8_empty(false)
        .auto_prefilter(true)
        .which_captures(WhichCaptures::All)
        .nfa_size_limit(None)
        .onepass_size_limit(Some(0)) // edge case of empty size
        .hybrid_cache_capacity(1) // minimum capacity
        .hybrid(true)
        .dfa(true)
        .line_terminator(0); // edge case of minimum byte

    let new_config = Config::new()
        .match_kind(MatchKind::LeftmostFirst)
        .utf8_empty(true)
        .auto_prefilter(false)
        .prefilter(Some(Prefilter {
            pre: Arc::new(/* implement PrefilterI */),
            is_fast: true,
            max_needle_len: 0, // edge case of zero needle length
        }))
        .which_captures(WhichCaptures::None)
        .nfa_size_limit(Some(usize::MAX))
        .onepass_size_limit(Some(usize::MAX))
        .hybrid_cache_capacity(usize::MAX)
        .hybrid(false)
        .dfa(false)
        .dfa_size_limit(Some(usize::MAX))
        .dfa_state_limit(Some(usize::MAX))
        .onepass(false)
        .backtrack(false)
        .byte_classes(false)
        .line_terminator(255); // edge case of maximum byte

    original_config.overwrite(new_config);
}

