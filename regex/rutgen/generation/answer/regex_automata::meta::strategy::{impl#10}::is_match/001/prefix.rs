// Answer 0

#[test]
fn test_is_match_anchored_yes() {
    let haystack: &[u8] = b"example input";
    let span = Span::new(0..haystack.len());
    let input = Input::new(haystack).span(span).anchored(Anchored::Yes);
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseInner { core, preinner: Prefilter::default(), nfarev: NFA::default(), hybrid: wrappers::ReverseHybrid::default(), dfa: wrappers::ReverseDFA::default() };

    strategy.is_match(&mut cache, &input);
}

#[test]
fn test_is_match_anchored_pattern() {
    let haystack: &[u8] = b"pattern match example";
    let span = Span::new(0..haystack.len());
    let pattern_id = PatternID::new(1);
    let input = Input::new(haystack).span(span).anchored(Anchored::Pattern(pattern_id));
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseInner { core, preinner: Prefilter::default(), nfarev: NFA::default(), hybrid: wrappers::ReverseHybrid::default(), dfa: wrappers::ReverseDFA::default() };

    strategy.is_match(&mut cache, &input);
}

#[test]
fn test_is_match_anchored_boundary() {
    let haystack: &[u8] = b"boundary match";
    let span = Span::new(0..haystack.len());
    let input = Input::new(haystack).span(span).anchored(Anchored::Yes);
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseInner { core, preinner: Prefilter::default(), nfarev: NFA::default(), hybrid: wrappers::ReverseHybrid::default(), dfa: wrappers::ReverseDFA::default() };

    strategy.is_match(&mut cache, &input);
}

