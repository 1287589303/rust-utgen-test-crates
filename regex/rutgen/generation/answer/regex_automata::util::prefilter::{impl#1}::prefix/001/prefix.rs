// Answer 0

#[test]
fn test_prefix_valid_span() {
    struct TestPrefilter;
    
    impl PrefilterI for TestPrefilter {
        fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> { Some(span) }
        fn memory_usage(&self) -> usize { 0 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = TestPrefilter;
    let haystack: &[u8] = b"Hello, world!";
    let valid_span = Span { start: 0, end: 5 };
    prefilter.prefix(haystack, valid_span);
}

#[test]
fn test_prefix_valid_span_boundary() {
    struct TestPrefilter;

    impl PrefilterI for TestPrefilter {
        fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> { Some(span) }
        fn memory_usage(&self) -> usize { 0 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = TestPrefilter;
    let haystack: &[u8] = b"Hello, world!";
    let boundary_span = Span { start: 0, end: 13 };
    prefilter.prefix(haystack, boundary_span);
}

#[test]
#[should_panic]
fn test_prefix_invalid_span_start_equals_end() {
    struct TestPrefilter;
    
    impl PrefilterI for TestPrefilter {
        fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> { Some(span) }
        fn memory_usage(&self) -> usize { 0 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = TestPrefilter;
    let haystack: &[u8] = b"Hello, world!";
    let invalid_span = Span { start: 5, end: 5 }; // invalid span
    prefilter.prefix(haystack, invalid_span);
}

#[test]
#[should_panic]
fn test_prefix_out_of_bounds_start() {
    struct TestPrefilter;
    
    impl PrefilterI for TestPrefilter {
        fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> { Some(span) }
        fn memory_usage(&self) -> usize { 0 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = TestPrefilter;
    let haystack: &[u8] = b"Hello, world!";
    let out_of_bounds_span = Span { start: 14, end: 15 }; // start is out of bounds
    prefilter.prefix(haystack, out_of_bounds_span);
}

#[test]
#[should_panic]
fn test_prefix_out_of_bounds_end() {
    struct TestPrefilter;
    
    impl PrefilterI for TestPrefilter {
        fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> { Some(span) }
        fn memory_usage(&self) -> usize { 0 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = TestPrefilter;
    let haystack: &[u8] = b"Hello, world!";
    let out_of_bounds_end_span = Span { start: 5, end: 14 }; // valid
    prefilter.prefix(haystack, out_of_bounds_end_span);
}

