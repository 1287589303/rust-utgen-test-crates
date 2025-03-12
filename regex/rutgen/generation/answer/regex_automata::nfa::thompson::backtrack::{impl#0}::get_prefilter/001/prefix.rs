// Answer 0

#[test]
fn test_get_prefilter_none() {
    let config = Config::new();
    let result = config.get_prefilter();
}

#[test]
fn test_get_prefilter_some_none() {
    let config = Config::default().prefilter(Some(None));
    let result = config.get_prefilter();
}

#[test]
fn test_get_prefilter_some_some_fast() {
    struct TestPrefilter;
    impl PrefilterI for TestPrefilter {}

    let prefilter = Prefilter {
        pre: Arc::new(TestPrefilter),
        is_fast: true,
        max_needle_len: 0,
    };
    let config = Config::default().prefilter(Some(Some(prefilter)));
    let result = config.get_prefilter();
}

#[test]
fn test_get_prefilter_some_some_slow() {
    struct TestPrefilter;
    impl PrefilterI for TestPrefilter {}

    let prefilter = Prefilter {
        pre: Arc::new(TestPrefilter),
        is_fast: false,
        max_needle_len: 1024,
    };
    let config = Config::default().prefilter(Some(Some(prefilter)));
    let result = config.get_prefilter();
}

