// Answer 0

#[test]
fn test_is_fast_true() {
    struct PrefilterImpl {
        is_fast: bool,
    }

    impl Debug for PrefilterImpl {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PrefilterImpl")
        }
    }

    impl Send for PrefilterImpl {}
    impl Sync for PrefilterImpl {}
    impl RefUnwindSafe for PrefilterImpl {}
    impl UnwindSafe for PrefilterImpl {}

    let prefilter = Prefilter {
        pre: Arc::new(PrefilterImpl { is_fast: true }),
        is_fast: true,
        max_needle_len: 0,
    };

    prefilter.is_fast();
}

#[test]
fn test_is_fast_false() {
    struct PrefilterImpl {
        is_fast: bool,
    }

    impl Debug for PrefilterImpl {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PrefilterImpl")
        }
    }

    impl Send for PrefilterImpl {}
    impl Sync for PrefilterImpl {}
    impl RefUnwindSafe for PrefilterImpl {}
    impl UnwindSafe for PrefilterImpl {}

    let prefilter = Prefilter {
        pre: Arc::new(PrefilterImpl { is_fast: false }),
        is_fast: false,
        max_needle_len: 0,
    };

    prefilter.is_fast();
}

