// Answer 0

#[test]
fn test_prefilter_with_some_prefilter() {
    let pre = Prefilter {
        pre: Arc::new(Some(MockPrefilterI::default())),
        is_fast: true,
        max_needle_len: 0,
    };

    let config = Config::new();
    config.prefilter(Some(pre));
}

#[test]
fn test_prefilter_with_none() {
    let config = Config::new();
    config.prefilter(None);
}

#[derive(Default)]
struct MockPrefilterI;

