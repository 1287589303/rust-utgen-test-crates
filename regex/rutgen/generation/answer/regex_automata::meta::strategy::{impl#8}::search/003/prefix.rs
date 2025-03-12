// Answer 0

#[test]
fn test_search_half_start_err_quadratic() {
    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"example haystack"[..])
        .anchored(Anchored::No)
        .span(0..13);

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let reverse_suffix = ReverseSuffix { core, pre: Prefilter::default() };
    
    reverse_suffix.search(&mut cache, &input);
}

#[test]
fn test_search_half_start_err_fail() {
    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"sample text with patterns"[..])
        .anchored(Anchored::No)
        .span(0..24);

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let reverse_suffix = ReverseSuffix { core, pre: Prefilter::default() };

    reverse_suffix.search(&mut cache, &input);
}

#[test]
fn test_search_half_start_ok_none() {
    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"haystack with no match"[..])
        .anchored(Anchored::No)
        .span(0..23);

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let reverse_suffix = ReverseSuffix { core, pre: Prefilter::default() };

    reverse_suffix.search(&mut cache, &input);
}

#[test]
fn test_search_half_start_ok_some() {
    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"matching pattern found"[..])
        .anchored(Anchored::No)
        .span(0..23);

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let reverse_suffix = ReverseSuffix { core, pre: Prefilter::default() };

    reverse_suffix.search(&mut cache, &input);
}

