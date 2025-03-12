// Answer 0

#[test]
fn test_set_end_within_bounds() {
    let mut input = Input::new(b"foobar");
    input.set_end(6);
}

#[test]
fn test_set_end_at_zero_length_haystack() {
    let mut input = Input::new(b"");
    input.set_end(0);
}

#[test]
#[should_panic]
fn test_set_end_beyond_haystack_length() {
    let mut input = Input::new(b"foobar");
    input.set_end(7);
}

#[test]
#[should_panic]
fn test_set_end_leading_to_invalid_span() {
    let mut input = Input::new(b"foobar");
    input.set_end(0);
    input.set_span(Span { start: 1, end: 0 });
}

#[test]
fn test_set_end_with_exact_haystack_length() {
    let mut input = Input::new(b"foobar");
    input.set_end(6);
    assert_eq!(input.get_span(), Span { start: 0, end: 6 });
}

#[test]
fn test_set_end_to_start() {
    let mut input = Input::new(b"foobar");
    input.set_end(0);
    assert_eq!(input.get_span(), Span { start: 0, end: 0 });
}

