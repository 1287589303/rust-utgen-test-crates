// Answer 0

#[test]
fn test_search_half_no_anchored_valid_halfmatch() {
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack: &[u8] = b"tingling";
    let input = Input::new(haystack)
        .span(0..haystack.len())
        .anchored(Anchored::No)
        .earliest(false);

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: Prefilter::default() };

    let result = strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_err_quadratic() {
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack: &[u8] = b"tingling";
    let input = Input::new(haystack)
        .span(0..haystack.len())
        .anchored(Anchored::No)
        .earliest(false);

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: Prefilter::default() };

    // Simulate an error from try_search_half_start
    strategy.try_search_half_start(&mut cache, &input).unwrap_err();

    let result = strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_err_fail() {
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack: &[u8] = b"tingling";
    let input = Input::new(haystack)
        .span(0..haystack.len())
        .anchored(Anchored::No)
        .earliest(false);

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseSuffix { core, pre: Prefilter::default() };

    // Simulate an error from try_search_half_start
    strategy.try_search_half_start(&mut cache, &input).unwrap_err();

    let result = strategy.search_half(&mut cache, &input);
}

