// Answer 0

#[test]
fn test_search_half_anchored_yes() {
    let haystack: &[u8] = b"test haystack";
    let span = Span::new(0, haystack.len());
    let anchored = Anchored::Yes;

    let input = Input::new(haystack)
        .span(span)
        .anchored(anchored);

    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let reverse_suffix = ReverseSuffix {
        core,
        pre: Prefilter::default(),
    };

    let _ = reverse_suffix.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_anchored_pattern() {
    let haystack: &[u8] = b"example input";
    let span = Span::new(0, haystack.len());
    let pattern_id = PatternID::default();
    let anchored = Anchored::Pattern(pattern_id);

    let input = Input::new(haystack)
        .span(span)
        .anchored(anchored);

    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let reverse_suffix = ReverseSuffix {
        core,
        pre: Prefilter::default(),
    };

    let _ = reverse_suffix.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_anchored_reverse_suffix() {
    let haystack: &[u8] = b"searching for a match";
    let span = Span::new(0, haystack.len());
    let pattern_id = PatternID::default();
    let anchored = Anchored::Pattern(pattern_id);

    let input = Input::new(haystack)
        .span(span)
        .anchored(anchored);

    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let reverse_suffix = ReverseSuffix {
        core,
        pre: Prefilter::default(),
    };

    let _ = reverse_suffix.search_half(&mut cache, &input);
}

