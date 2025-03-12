// Answer 0

#[test]
fn test_search_slots_nofail_onepass() {
    let regex_info = RegexInfo(Arc::new(RegexInfoI::default()));
    let prefilter = Some(Prefilter {
        pre: Arc::new(MockPrefilter {}),
        is_fast: true,
        max_needle_len: 256,
    });

    let nfa = NFA(Arc::new(Inner::default()));
    let core = Core::new(regex_info.clone(), prefilter.clone(), &[]).unwrap();
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache(None),
        backtrack: wrappers::BoundedBacktrackerCache(None),
        onepass: wrappers::OnePassCache(Some(MockOnePassCache::default())),
        hybrid: wrappers::HybridCache(None),
        revhybrid: wrappers::ReverseHybridCache(None),
    };

    let input = Input {
        haystack: &vec![b'a'; 130],
        span: Span::default(),
        anchored: Anchored::True,
        earliest: false,
    };

    let mut slots = vec![None; 2];
    
    core.search_slots_nofail(&mut cache, &input, &mut slots);
}

#[test]
fn test_search_slots_nofail_backtrack() {
    let regex_info = RegexInfo(Arc::new(RegexInfoI::default()));
    let prefilter = Some(Prefilter {
        pre: Arc::new(MockPrefilter {}),
        is_fast: false,
        max_needle_len: 256,
    });

    let nfa = NFA(Arc::new(Inner::default()));
    let core = Core::new(regex_info.clone(), prefilter.clone(), &[]).unwrap();
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache(None),
        backtrack: wrappers::BoundedBacktrackerCache(Some(MockBacktrackCache::default())),
        onepass: wrappers::OnePassCache(None),
        hybrid: wrappers::HybridCache(None),
        revhybrid: wrappers::ReverseHybridCache(None),
    };

    let input = Input {
        haystack: &vec![b'b'; 130],
        span: Span::default(),
        anchored: Anchored::False,
        earliest: false,
    };

    let mut slots = vec![None; 2];

    core.search_slots_nofail(&mut cache, &input, &mut slots);
}

