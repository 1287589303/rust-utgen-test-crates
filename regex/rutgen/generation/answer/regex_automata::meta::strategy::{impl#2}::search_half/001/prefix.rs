// Answer 0

#[test]
fn test_search_half_valid_input() {
    let haystack: &[u8] = b"example";
    let span = Span::new(0, 7);
    let anchored = Anchored::Yes;
    let input = Input {
        haystack,
        span,
        anchored,
        earliest: true,
    };

    struct DummyPrefilter;
    impl PrefilterI for DummyPrefilter {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> {
            None
        }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> {
            None
        }
        fn memory_usage(&self) -> usize {
            0
        }
        fn is_fast(&self) -> bool {
            true
        }
    }

    let group_info = GroupInfo::default();
    let prefilter = DummyPrefilter;
    let strategy = Pre { pre: prefilter, group_info };
    let mut cache = strategy.create_cache();

    let _ = strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_empty_haystack() {
    let haystack: &[u8] = b"";
    let span = Span::new(0, 0);
    let anchored = Anchored::No;
    let input = Input {
        haystack,
        span,
        anchored,
        earliest: true,
    };

    struct DummyPrefilter;
    impl PrefilterI for DummyPrefilter {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> {
            None
        }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> {
            None
        }
        fn memory_usage(&self) -> usize {
            0
        }
        fn is_fast(&self) -> bool {
            true
        }
    }

    let group_info = GroupInfo::default();
    let prefilter = DummyPrefilter;
    let strategy = Pre { pre: prefilter, group_info };
    let mut cache = strategy.create_cache();

    let _ = strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_single_byte_haystack() {
    let haystack: &[u8] = b"a";
    let span = Span::new(0, 1);
    let anchored = Anchored::Yes;
    let input = Input {
        haystack,
        span,
        anchored,
        earliest: true,
    };

    struct DummyPrefilter;
    impl PrefilterI for DummyPrefilter {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> {
            None
        }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> {
            None
        }
        fn memory_usage(&self) -> usize {
            0
        }
        fn is_fast(&self) -> bool {
            true
        }
    }

    let group_info = GroupInfo::default();
    let prefilter = DummyPrefilter;
    let strategy = Pre { pre: prefilter, group_info };
    let mut cache = strategy.create_cache();

    let _ = strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_anchored_no() {
    let haystack: &[u8] = b"test string";
    let span = Span::new(0, 10);
    let anchored = Anchored::No;
    let input = Input {
        haystack,
        span,
        anchored,
        earliest: false,
    };

    struct DummyPrefilter;
    impl PrefilterI for DummyPrefilter {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> {
            None
        }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> {
            None
        }
        fn memory_usage(&self) -> usize {
            0
        }
        fn is_fast(&self) -> bool {
            true
        }
    }

    let group_info = GroupInfo::default();
    let prefilter = DummyPrefilter;
    let strategy = Pre { pre: prefilter, group_info };
    let mut cache = strategy.create_cache();

    let _ = strategy.search_half(&mut cache, &input);
}

