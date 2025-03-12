// Answer 0

#[test]
fn test_pattern_no() {
    let anchored_search = Anchored::No;
    let result = anchored_search.pattern();
}

#[test]
fn test_pattern_yes() {
    let anchored_search = Anchored::Yes;
    let result = anchored_search.pattern();
}

