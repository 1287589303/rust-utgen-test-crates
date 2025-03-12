// Answer 0

#[test]
fn test_is_accelerated_with_fast_prefilter() {
    let prefilter = Prefilter {
        #[cfg(not(feature = "alloc"))]
        _unused: (),
        #[cfg(feature = "alloc")]
        pre: Arc::new(MockPrefilter { is_fast: true }),
        #[cfg(feature = "alloc")]
        is_fast: true,
        #[cfg(feature = "alloc")]
        max_needle_len: 10,
    };
    
    let strategy = ReverseSuffix {
        core: Core {
            info: RegexInfo::default(),
            pre: Some(prefilter),
            nfa: NFA::default(),
            nfarev: None,
            pikevm: wrappers::PikeVM::default(),
            backtrack: wrappers::BoundedBacktracker::default(),
            onepass: wrappers::OnePass::default(),
            hybrid: wrappers::Hybrid::default(),
            dfa: wrappers::DFA::default(),
        },
        pre: Prefilter::default(), // Just to satisfy struct initialization.
    };

    strategy.is_accelerated();
}

#[test]
fn test_is_accelerated_with_non_fast_prefilter() {
    let prefilter = Prefilter {
        #[cfg(not(feature = "alloc"))]
        _unused: (),
        #[cfg(feature = "alloc")]
        pre: Arc::new(MockPrefilter { is_fast: false }),
        #[cfg(feature = "alloc")]
        is_fast: false,
        #[cfg(feature = "alloc")]
        max_needle_len: 10,
    };
    
    let strategy = ReverseSuffix {
        core: Core {
            info: RegexInfo::default(),
            pre: Some(prefilter),
            nfa: NFA::default(),
            nfarev: None,
            pikevm: wrappers::PikeVM::default(),
            backtrack: wrappers::BoundedBacktracker::default(),
            onepass: wrappers::OnePass::default(),
            hybrid: wrappers::Hybrid::default(),
            dfa: wrappers::DFA::default(),
        },
        pre: Prefilter::default(), // Just to satisfy struct initialization.
    };

    strategy.is_accelerated();
}

struct MockPrefilter {
    is_fast: bool,
}

impl PrefilterI for MockPrefilter {
    fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
    fn prefix(&self, _: &[u8], _: Span) -> Option<Span> { None }
    fn memory_usage(&self) -> usize { 0 }
    fn max_needle_len(&self) -> usize { 0 }
    fn is_fast(&self) -> bool { self.is_fast }
}

