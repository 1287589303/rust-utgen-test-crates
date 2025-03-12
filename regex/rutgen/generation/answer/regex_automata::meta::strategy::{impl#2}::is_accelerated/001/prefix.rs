// Answer 0

#[test]
fn test_is_accelerated_with_fast_prefilter() {
    #[derive(Debug)]
    struct FastPrefilter;

    impl PrefilterI for FastPrefilter {
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

    let pre = Pre {
        pre: FastPrefilter,
        group_info: GroupInfo(Arc::new(GroupInfoInner)),
    };

    let result = pre.is_accelerated();
}

#[test]
fn test_is_accelerated_with_non_fast_prefilter() {
    #[derive(Debug)]
    struct SlowPrefilter;

    impl PrefilterI for SlowPrefilter {
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
            false
        }
    }

    let pre = Pre {
        pre: SlowPrefilter,
        group_info: GroupInfo(Arc::new(GroupInfoInner)),
    };

    let result = pre.is_accelerated();
}

