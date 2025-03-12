// Answer 0

#[test]
fn test_lazy_ref_new_valid_inputs() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1024,
    };

    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let _lazy_ref = LazyRef::new(&dfa, &cache);
}

#[test]
fn test_lazy_ref_new_another_valid_inputs() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 16,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 512,
    };

    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let _lazy_ref = LazyRef::new(&dfa, &cache);
}

#[test]
#[should_panic]
fn test_lazy_ref_new_null_dfa() {
    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let _lazy_ref = LazyRef::new(std::ptr::null(), &cache);
}

#[test]
#[should_panic]
fn test_lazy_ref_new_null_cache() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1024,
    };

    let _lazy_ref = LazyRef::new(&dfa, std::ptr::null());
}

