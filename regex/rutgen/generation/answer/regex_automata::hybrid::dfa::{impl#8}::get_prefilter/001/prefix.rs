// Answer 0

#[test]
fn test_get_prefilter_none() {
    let config = Config::default();
    let prefilter = config.get_prefilter();
}

#[test]
fn test_get_prefilter_some() {
    #[cfg(feature = "alloc")]
    struct TestPrefilter {
        is_fast: bool,
        max_needle_len: usize,
    }

    #[cfg(feature = "alloc")]
    impl TestPrefilter {
        fn new(is_fast: bool, max_needle_len: usize) -> Self {
            TestPrefilter { is_fast, max_needle_len }
        }
    }

    #[cfg(feature = "alloc")]
    let my_prefilter = Prefilter {
        pre: Arc::new(TestPrefilter::new(true, 5)),
        is_fast: true,
        max_needle_len: 10,
    };

    let config = Config::default().prefilter(Some(my_prefilter));
    let prefilter = config.get_prefilter();
}

