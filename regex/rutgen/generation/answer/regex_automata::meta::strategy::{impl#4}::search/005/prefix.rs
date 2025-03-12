// Answer 0

#[test]
fn test_search_with_full_dfa_success() {
    let info = RegexInfo(Arc::new(RegexInfoI::default()));
    let nfa = NFA(Arc::new(Inner::default()));
    let nfarev = NFA(Arc::new(Inner::default()));
    let prefilter = Some(Prefilter {
        pre: Arc::new(PrefilterI::default()),
        is_fast: true,
        max_needle_len: 256,
    });

    let core = Core::new(info.clone(), prefilter.clone(), &[]).unwrap();
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input {
        haystack: b"valid input".as_ref(),
        span: Span::new(0, 11),
        anchored: Anchored::Yes,
        earliest: true,
    };

    core.search(&mut cache, &input);
}

#[test]
fn test_search_with_full_dfa_success_anchored_no() {
    let info = RegexInfo(Arc::new(RegexInfoI::default()));
    let nfa = NFA(Arc::new(Inner::default()));
    let nfarev = NFA(Arc::new(Inner::default()));
    let prefilter = Some(Prefilter {
        pre: Arc::new(PrefilterI::default()),
        is_fast: false,
        max_needle_len: 128,
    });

    let core = Core::new(info.clone(), prefilter.clone(), &[]).unwrap();
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input {
        haystack: b"another valid input".as_ref(),
        span: Span::new(0, 19),
        anchored: Anchored::No,
        earliest: false,
    };

    core.search(&mut cache, &input);
}

