// Answer 0

#[test]
fn test_set_anchored_no() {
    let mut input = Input::new("foobar");
    input.set_anchored(Anchored::No);
}

#[test]
fn test_set_anchored_yes() {
    let mut input = Input::new("foobar");
    input.set_anchored(Anchored::Yes);
}

#[test]
fn test_set_anchored_pattern() {
    let mut input = Input::new("foobar");
    let pid = PatternID::must(1);
    input.set_anchored(Anchored::Pattern(pid));
}

#[test]
fn test_set_anchored_pattern_boundary() {
    let mut input = Input::new("foobar");
    let pid = PatternID::must(usize::MAX);
    input.set_anchored(Anchored::Pattern(pid));
}

