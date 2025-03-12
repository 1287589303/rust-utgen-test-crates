// Answer 0

#[test]
fn test_get_look_matcher_default() {
    let builder = Builder::default();
    let matcher = builder.get_look_matcher();
}

#[test]
fn test_get_look_matcher_explicitly_set() {
    let mut builder = Builder::default();
    let look_matcher = LookMatcher {
        lineterm: DebugByte::default(),
    };
    builder.set_look_matcher(look_matcher);
    let matcher = builder.get_look_matcher();
}

