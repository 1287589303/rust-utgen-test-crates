// Answer 0

#[test]
fn test_is_match_with_fail_error() {
    let haystack: &[u8] = b"some input text";
    let span = 0..haystack.len();
    let input = Input::new(haystack)
        .span(span)
        .anchored(Anchored::No)
        .earliest(false);
    
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseInner::new(core, &[]).unwrap();

    strategy.is_match(&mut cache, &input);
}

#[test]
fn test_is_match_with_quadratic_error() {
    let haystack: &[u8] = b"some other input text";
    let span = 0..haystack.len();
    let input = Input::new(haystack)
        .span(span)
        .anchored(Anchored::No)
        .earliest(true);
    
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseInner::new(core, &[]).unwrap();

    strategy.is_match(&mut cache, &input);
}

