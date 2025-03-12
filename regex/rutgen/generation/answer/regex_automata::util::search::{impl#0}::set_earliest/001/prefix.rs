// Answer 0

#[test]
fn test_set_earliest_true() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_earliest(true);
}

#[test]
fn test_set_earliest_false() {
    let mut input = Input::new(&b"test"[..]);
    input.set_earliest(false);
}

#[test]
fn test_set_earliest_boundary() {
    let mut input = Input::new(&b"a"[..]);
    input.set_earliest(true);
    input.set_earliest(false);
}

#[test]
fn test_set_earliest_multiple() {
    let mut input = Input::new(&b"example"[..]);
    input.set_earliest(true);
    input.set_earliest(true);
    input.set_earliest(false);
}

#[test]
fn test_set_earliest_with_span() {
    let mut input = Input::new(&b"longer example"[..]).span(Span { start: 0, end: 14 });
    input.set_earliest(true);
} 

#[test]
fn test_set_earliest_with_anchored_no() {
    let mut input = Input::new(&b"byte array"[..]).anchored(Anchored::No);
    input.set_earliest(false);
}

#[test]
fn test_set_earliest_with_anchored_yes() {
    let mut input = Input::new(&b"more tests"[..]).anchored(Anchored::Yes);
    input.set_earliest(true);
}

#[test]
fn test_set_earliest_with_anchored_pattern() {
    let mut input = Input::new(&b"pattern test"[..]).anchored(Anchored::Pattern(1));
    input.set_earliest(true);
}

