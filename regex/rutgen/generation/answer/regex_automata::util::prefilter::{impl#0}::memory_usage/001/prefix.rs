// Answer 0

#[test]
fn test_memory_usage_zero() {
    struct TestPrefilter;
    impl PrefilterI for TestPrefilter {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { 0 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = Prefilter {
        pre: Arc::new(TestPrefilter),
        is_fast: true,
        max_needle_len: 10,
    };

    let _ = prefilter.memory_usage();
}

#[test]
fn test_memory_usage_max() {
    struct TestPrefilter;
    impl PrefilterI for TestPrefilter {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { 10000 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = Prefilter {
        pre: Arc::new(TestPrefilter),
        is_fast: true,
        max_needle_len: 10,
    };

    let _ = prefilter.memory_usage();
}

#[test]
fn test_memory_usage_mid_range() {
    struct TestPrefilter;
    impl PrefilterI for TestPrefilter {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { 5000 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = Prefilter {
        pre: Arc::new(TestPrefilter),
        is_fast: true,
        max_needle_len: 10,
    };

    let _ = prefilter.memory_usage();
}

