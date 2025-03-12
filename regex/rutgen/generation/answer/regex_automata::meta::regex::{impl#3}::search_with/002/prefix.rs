// Answer 0

#[test]
fn test_search_with_valid_input_anchored() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };
    let haystack = b"Samwise the Brave";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::True,
        earliest: false,
    };
    let regex = Regex {
        imp: Arc::new(RegexI {
            strat: Arc::new(strategy::SomeStrategy::new()),
            info: RegexInfo::new(Config::default(), &[]),
        }),
        pool: Pool::new(),
    };
    regex.search_with(&mut cache, &input);
}

#[test]
fn test_search_with_valid_input_non_anchored() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };
    let haystack = b"This is a test string containing Sam";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::False,
        earliest: true,
    };
    let regex = Regex {
        imp: Arc::new(RegexI {
            strat: Arc::new(strategy::SomeStrategy::new()),
            info: RegexInfo::new(Config::default(), &[]),
        }),
        pool: Pool::new(),
    };
    regex.search_with(&mut cache, &input);
}

#[test]
fn test_search_with_boundary_input() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };
    let haystack = b"A";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::False,
        earliest: true,
    };
    let regex = Regex {
        imp: Arc::new(RegexI {
            strat: Arc::new(strategy::SomeStrategy::new()),
            info: RegexInfo::new(Config::default(), &[]),
        }),
        pool: Pool::new(),
    };
    regex.search_with(&mut cache, &input);
}

#[test]
fn test_search_with_empty_haystack() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };
    let haystack = b"";
    let span = Span::new(0, 0);
    let input = Input {
        haystack,
        span,
        anchored: Anchored::False,
        earliest: true,
    };
    let regex = Regex {
        imp: Arc::new(RegexI {
            strat: Arc::new(strategy::SomeStrategy::new()),
            info: RegexInfo::new(Config::default(), &[]),
        }),
        pool: Pool::new(),
    };
    regex.search_with(&mut cache, &input);
}

