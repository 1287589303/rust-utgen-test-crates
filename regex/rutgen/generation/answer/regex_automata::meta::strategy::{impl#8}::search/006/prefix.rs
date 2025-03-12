// Answer 0

#[test]
fn test_search_anchored_false_ok_none() {
    let haystack: &[u8] = b"test input string";
    let span = Span::new(0, haystack.len());
    let input = Input::new(haystack).span(span).anchored(Anchored::No);
    
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let prefilter = Prefilter {
        pre: Arc::new(MyPrefilter {}),
        is_fast: true,
        max_needle_len: 10,
    };

    let core = Core::new(RegexInfo::default(), Some(prefilter), &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: prefilter };

    let result = strategy.search(&mut cache, &input);
}

#[test]
fn test_search_anchored_false_ok_some() {
    let haystack: &[u8] = b"another test string";
    let span = Span::new(0, haystack.len());
    let input = Input::new(haystack).span(span).anchored(Anchored::No);
    
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let prefilter = Prefilter {
        pre: Arc::new(MyPrefilter {}),
        is_fast: true,
        max_needle_len: 10,
    };

    let core = Core::new(RegexInfo::default(), Some(prefilter), &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: prefilter };

    let result = strategy.search(&mut cache, &input);
}

#[test]
#[should_panic]
fn test_search_fail_error() {
    let haystack: &[u8] = b"failing input case";
    let span = Span::new(0, haystack.len());
    let input = Input::new(haystack).span(span).anchored(Anchored::No);
    
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let prefilter = Prefilter {
        pre: Arc::new(MyPrefilter {}),
        is_fast: true,
        max_needle_len: 10,
    };

    let core = Core::new(RegexInfo::default(), Some(prefilter), &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: prefilter };

    let result = strategy.search(&mut cache, &input);
}

#[test]
fn test_search_fwdinput() {
    let haystack: &[u8] = b"searchable input";
    let span = Span::new(0, haystack.len());
    let input = Input::new(haystack).span(span).anchored(Anchored::No);
    
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let prefilter = Prefilter {
        pre: Arc::new(MyPrefilter {}),
        is_fast: true,
        max_needle_len: 10,
    };

    let core = Core::new(RegexInfo::default(), Some(prefilter), &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: prefilter };
    
    let hm_start = HalfMatch::new(PatternID(0), 15);
    let fwdinput = input.clone().anchored(Anchored::Pattern(hm_start.pattern())).span(hm_start.offset()..input.end());

    let result = strategy.search(&mut cache, &fwdinput);
}

