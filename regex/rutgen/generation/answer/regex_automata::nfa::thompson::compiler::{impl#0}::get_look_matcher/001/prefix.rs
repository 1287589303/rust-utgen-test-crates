// Answer 0

#[test]
fn test_get_look_matcher_none() {
    let config = Config::default();
    let matcher = config.get_look_matcher();
}

#[test]
fn test_get_look_matcher_some() {
    let config = Config::default().look_matcher(LookMatcher::default());
    let matcher = config.get_look_matcher();
}

