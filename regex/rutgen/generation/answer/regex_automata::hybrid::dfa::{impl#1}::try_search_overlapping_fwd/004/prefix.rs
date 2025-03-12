// Answer 0

#[test]
fn test_try_search_overlapping_fwd_no_match() {
    let dfa = {
        #[cfg(feature = "syntax")]
        let nfa = NFA::new(r"\w*")?;
        #[cfg(not(feature = "syntax"))]
        let nfa = NFA::never_match();

        DFA {
            config: Config {
                match_kind: Some(MatchKind::All),
                ..Default::default()
            },
            nfa,
            stride2: 0,
            start_map: StartByteMap { map: [Start::default(); 256] },
            classes: ByteClasses([0; 256]),
            quitset: ByteSet::default(),
            cache_capacity: 0,
        }
    };

    let mut cache = dfa.create_cache();
    let input = Input {
        haystack: b"notmatchingtext",
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };
    
    let mut state = OverlappingState::start();
    
    let result = dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state);
    assert!(result.is_ok());
} 

#[test]
fn test_try_search_overlapping_fwd_empty_utf8() {
    let dfa = {
        #[cfg(feature = "syntax")]
        let nfa = NFA::new(r"\w*")?;
        #[cfg(not(feature = "syntax"))]
        let nfa = NFA::never_match();

        DFA {
            config: Config {
                match_kind: Some(MatchKind::All),
                ..Default::default()
            },
            nfa,
            stride2: 0,
            start_map: StartByteMap { map: [Start::default(); 256] },
            classes: ByteClasses([0; 256]),
            quitset: ByteSet::default(),
            cache_capacity: 0,
        }
    };

    // Ensuring doesn't match for the empty state
    let mut cache = dfa.create_cache();
    let input = Input {
        haystack: b"empty input",
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };

    let mut state = OverlappingState::start();
    
    let result = dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state);
    assert!(result.is_ok());
}

