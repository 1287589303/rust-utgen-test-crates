// Answer 0

#[test]
fn test_search_half_with_dfa() {
    let regex_info = RegexInfo(Arc::new(RegexInfoI::new()));
    let nfa = NFA::new();
    let prefilter = Some(Prefilter {
        pre: Arc::new(MyPrefilter {}),
        is_fast: true,
        max_needle_len: 64,
    });
    let core = Core::new(regex_info.clone(), prefilter.clone(), &[]).unwrap();
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let input = Input {
        haystack: b"test input".to_vec(),
        span: Span::new(0, 10),
        anchored: Anchored::No,
        earliest: true,
    };

    if let Some(e) = core.dfa.get(&input) {
        let result = e.try_search_half_fwd(&input);
        assert!(result.is_ok());
        let half_match = core.search_half(&mut cache, &input);
        assert!(half_match.is_some());
    }
}

#[test]
fn test_search_half_with_hybrid() {
    let regex_info = RegexInfo(Arc::new(RegexInfoI::new()));
    let nfa = NFA::new();
    let prefilter = Some(Prefilter {
        pre: Arc::new(MyPrefilter {}),
        is_fast: true,
        max_needle_len: 64,
    });
    let core = Core::new(regex_info.clone(), prefilter.clone(), &[]).unwrap();
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let input = Input {
        haystack: b"another test input".to_vec(),
        span: Span::new(0, 17),
        anchored: Anchored::No,
        earliest: true,
    };

    if let Some(e) = core.hybrid.get(&input) {
        let result = e.try_search_half_fwd(&mut cache.hybrid, &input);
        assert!(result.is_ok());
        let half_match = core.search_half(&mut cache, &input);
        assert!(half_match.is_some());
    }
}

