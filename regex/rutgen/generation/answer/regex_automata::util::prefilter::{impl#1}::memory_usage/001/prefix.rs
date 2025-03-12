// Answer 0

#[test]
fn test_memory_usage_non_empty() {
    struct TestPrefilter;

    impl Debug for TestPrefilter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestPrefilter")
        }
    }

    impl PrefilterI for TestPrefilter {
        fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn prefix(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { 42 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = Arc::new(TestPrefilter);
    let usage = prefilter.memory_usage();
}

#[test]
fn test_memory_usage_empty() {
    struct TestEmptyPrefilter;

    impl Debug for TestEmptyPrefilter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestEmptyPrefilter")
        }
    }

    impl PrefilterI for TestEmptyPrefilter {
        fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn prefix(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { 0 }
        fn is_fast(&self) -> bool { false }
    }

    let prefilter = Arc::new(TestEmptyPrefilter);
    let usage = prefilter.memory_usage();
}

#[test]
fn test_memory_usage_large() {
    struct TestLargePrefilter;

    impl Debug for TestLargePrefilter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestLargePrefilter")
        }
    }

    impl PrefilterI for TestLargePrefilter {
        fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn prefix(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { 65536 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = Arc::new(TestLargePrefilter);
    let usage = prefilter.memory_usage();
}

