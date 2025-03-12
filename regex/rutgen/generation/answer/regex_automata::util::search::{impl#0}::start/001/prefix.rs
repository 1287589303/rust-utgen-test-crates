// Answer 0

#[test]
fn test_start_default_case() {
    let input = Input::new(b"foobar");
    let _ = input.start();
}

#[test]
fn test_start_with_span_valid_range() {
    let input = Input::new(b"foobar").span(2..4);
    let _ = input.start();
}

#[test]
fn test_start_with_span_zero_range() {
    let input = Input::new(b"foobar").span(0..0);
    let _ = input.start();
}

#[test]
fn test_start_with_span_full_range() {
    let input = Input::new(b"foobar").span(0..6);
    let _ = input.start();
}

#[test]
fn test_start_with_anchored_yes() {
    let input = Input::new(b"foobar").anchored(Anchored::Yes);
    let _ = input.start();
}

#[test]
fn test_start_with_anchored_no() {
    let input = Input::new(b"foobar").anchored(Anchored::No);
    let _ = input.start();
}

#[test]
fn test_start_with_anchored_pattern() {
    let input = Input::new(b"foobar").anchored(Anchored::Pattern(PatternID(1)));
    let _ = input.start();
}

#[test]
fn test_start_with_earliest_true() {
    let input = Input::new(b"foobar").earliest(true);
    let _ = input.start();
}

#[test]
fn test_start_with_earliest_false() {
    let input = Input::new(b"foobar").earliest(false);
    let _ = input.start();
}

#[test]
fn test_start_empty_haystack() {
    let input = Input::new(b"").span(0..0);
    let _ = input.start();
}

