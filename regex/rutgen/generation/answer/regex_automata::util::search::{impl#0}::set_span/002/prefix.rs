// Answer 0

#[test]
#[should_panic]
fn test_set_span_start_greater_than_end() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_span(3..2);
}

#[test]
#[should_panic]
fn test_set_span_start_equals_end() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_span(3..3);
}

#[test]
fn test_set_span_boundary_case() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_span(6..6);
}

