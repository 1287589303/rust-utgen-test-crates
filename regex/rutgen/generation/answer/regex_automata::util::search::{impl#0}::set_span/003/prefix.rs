// Answer 0

#[test]
#[should_panic]
fn test_set_span_end_exceeds_haystack_length() {
    let mut input = Input::new("foo");
    input.set_span(0..4);
}

#[test]
#[should_panic]
fn test_set_span_start_greater_than_end() {
    let mut input = Input::new("foobar");
    input.set_span(4..2);
}

#[test]
#[should_panic]
fn test_set_span_start_exceeds_haystack_length() {
    let mut input = Input::new("test");
    input.set_span(5..6);
}

