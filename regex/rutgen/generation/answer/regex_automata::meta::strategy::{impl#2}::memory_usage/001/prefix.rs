// Answer 0

#[test]
fn test_memory_usage_zero() {
    #[derive(Clone, Debug)]
    struct PrefilterZero;

    impl PrefilterI for PrefilterZero {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { 0 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = PrefilterZero;
    let group_info = GroupInfo(Arc::new(GroupInfoInner));
    let strategy = Pre { pre: prefilter, group_info };

    strategy.memory_usage();
}

#[test]
fn test_memory_usage_one() {
    #[derive(Clone, Debug)]
    struct PrefilterOne;

    impl PrefilterI for PrefilterOne {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { 1 }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = PrefilterOne;
    let group_info = GroupInfo(Arc::new(GroupInfoInner));
    let strategy = Pre { pre: prefilter, group_info };

    strategy.memory_usage();
}

#[test]
fn test_memory_usage_large() {
    #[derive(Clone, Debug)]
    struct PrefilterLarge;

    impl PrefilterI for PrefilterLarge {
        fn find(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn prefix(&self, _haystack: &[u8], _span: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { usize::MAX }
        fn is_fast(&self) -> bool { true }
    }

    let prefilter = PrefilterLarge;
    let group_info = GroupInfo(Arc::new(GroupInfoInner));
    let strategy = Pre { pre: prefilter, group_info };

    strategy.memory_usage();
}

