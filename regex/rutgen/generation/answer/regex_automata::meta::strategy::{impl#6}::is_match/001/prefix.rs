// Answer 0

#[test]
fn test_is_match_with_anchored_yes() {
    let cache = &mut Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
  
    let input = Input::new(b"test input")
        .anchored(Anchored::Yes)
        .earliest(true);

    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseAnchored { core };
  
    strategy.is_match(cache, &input);
}

#[test]
fn test_is_match_with_anchored_pattern() {
    let pattern_id = PatternID::default();
    let cache = &mut Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(b"test input")
        .anchored(Anchored::Pattern(pattern_id))
        .earliest(false);
    
    let core = Core::new(RegexInfo::default(), None, &[]).unwrap();
    let strategy = ReverseAnchored { core };

    strategy.is_match(cache, &input);
}

