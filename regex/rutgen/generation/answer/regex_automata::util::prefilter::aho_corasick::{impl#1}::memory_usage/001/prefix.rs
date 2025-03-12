// Answer 0

#[test]
fn test_memory_usage_without_perf_literal_multisubstring() {
    struct TestPrefilter;
    
    impl Debug for TestPrefilter {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result { Ok(()) }
    }
    
    impl PrefilterI for TestPrefilter {
        fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn prefix(&self, _: &[u8], _: Span) -> Option<Span> { None }
        fn memory_usage(&self) -> usize { unreachable!() }
        fn is_fast(&self) -> bool { false }
    }

    let prefilter = TestPrefilter;
    let _ = prefilter.memory_usage();
}

#[test]
fn test_memory_usage_with_perf_literal_multisubstring() {
    #[cfg(feature = "perf-literal-multisubstring")]
    {
        struct TestAhoCorasick {
            memory_usage_value: usize,
        }
        
        impl TestAhoCorasick {
            fn memory_usage(&self) -> usize {
                self.memory_usage_value
            }
        }

        #[cfg(feature = "perf-literal-multisubstring")]
        impl PrefilterI for AhoCorasick {
            fn find(&self, _: &[u8], _: Span) -> Option<Span> { None }
            fn prefix(&self, _: &[u8], _: Span) -> Option<Span> { None }
            fn memory_usage(&self) -> usize { self.ac.memory_usage() }
            fn is_fast(&self) -> bool { true }
        }

        let ac = TestAhoCorasick { memory_usage_value: 0 };
        let prefilter = AhoCorasick { ac: ac };
        let _ = prefilter.memory_usage();

        let ac_high = TestAhoCorasick { memory_usage_value: usize::MAX };
        let prefilter_high = AhoCorasick { ac: ac_high };
        let _ = prefilter_high.memory_usage();
    }
}

