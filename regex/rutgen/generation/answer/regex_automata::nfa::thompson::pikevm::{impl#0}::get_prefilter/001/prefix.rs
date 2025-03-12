// Answer 0

#[test]
fn test_get_prefilter_some_fast() {
    let prefilter = Prefilter {
        pre: Arc::new(()), 
        is_fast: true,
        max_needle_len: 10,
    };
    let config = Config::new().prefilter(Some(prefilter));
    let _ = config.get_prefilter();
}

#[test]
fn test_get_prefilter_some_not_fast() {
    let prefilter = Prefilter {
        pre: Arc::new(()), 
        is_fast: false,
        max_needle_len: 20,
    };
    let config = Config::new().prefilter(Some(prefilter));
    let _ = config.get_prefilter();
}

#[test]
fn test_get_prefilter_with_min_max_needle_len() {
    let prefilter_min = Prefilter {
        pre: Arc::new(()), 
        is_fast: true,
        max_needle_len: 0,
    };
    let config_min = Config::new().prefilter(Some(prefilter_min));
    let _ = config_min.get_prefilter();

    let prefilter_max = Prefilter {
        pre: Arc::new(()), 
        is_fast: false,
        max_needle_len: usize::MAX,
    };
    let config_max = Config::new().prefilter(Some(prefilter_max));
    let _ = config_max.get_prefilter();
}

#[test]
fn test_get_prefilter_none() {
    let config = Config::new().prefilter(None);
    let _ = config.get_prefilter();
}

#[test]
fn test_get_prefilter_empty() {
    let config = Config::new();
    let _ = config.get_prefilter();
}

