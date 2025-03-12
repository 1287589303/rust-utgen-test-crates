// Answer 0

#[test]
fn test_set_span_valid_bounds_end_equal() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_span(6..6);
}

#[test]
fn test_set_span_valid_bounds_start_equal() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_span(5..6);
}

#[test]
fn test_set_span_valid_bounds_start_one_more() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_span(6..7);
}

