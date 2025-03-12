// Answer 0

#[test]
fn test_search_half_no_half_match() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    
    let haystack = b"short string";
    let input = Input::new(haystack).span(0..0).anchored(Anchored::No);
    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: Prefilter::default() };

    let result = strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_quad_retry() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack = b"another test";
    let input = Input::new(haystack).span(0..5).anchored(Anchored::No);
    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: Prefilter::default() };

    let hm_start = HalfMatch::new(PatternID(0), 0);
    strategy.try_search_half_start(&mut cache, &input).expect("Expected to find half match");

    let result = strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_fail_retry() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack = b"yet another";
    let input = Input::new(haystack).span(0..8).anchored(Anchored::No);
    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: Prefilter::default() };

    let hm_start = HalfMatch::new(PatternID(1), 5);
    strategy.try_search_half_start(&mut cache, &input).expect("Expected to find half match");

    let fwdinput = input.clone().anchored(Anchored::Pattern(hm_start.pattern()));
    let result = strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_fwd_no_match() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack = b"no match here";
    let input = Input::new(haystack).span(1..5).anchored(Anchored::No);
    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: Prefilter::default() };

    let hm_start = HalfMatch::new(PatternID(0), 1);
    strategy.try_search_half_start(&mut cache, &input).expect("Expected to find half match");

    let fwdinput = input.clone().anchored(Anchored::Pattern(hm_start.pattern())).span(1..7);
    let result = strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_fwd_with_match() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack = b"string matching";
    let input = Input::new(haystack).span(0..8).anchored(Anchored::No);
    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: Prefilter::default() };

    let hm_start = HalfMatch::new(PatternID(0), 2);
    strategy.try_search_half_start(&mut cache, &input).expect("Expected to find half match");

    let fwdinput = input.clone().anchored(Anchored::Pattern(hm_start.pattern())).span(2..12);
    let result = strategy.search_half(&mut cache, &input);
}

