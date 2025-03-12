// Answer 0

#[test]
fn test_input_from_single_byte_haystack() {
    let haystack: &[u8] = b"a";
    let span = Span { start: 0, end: 1 };
    let anchored = Anchored::No;
    let input = Input { haystack, span, anchored, earliest: false };
    let _ = Input::from(haystack);
}

#[test]
fn test_input_from_two_byte_haystack() {
    let haystack: &[u8] = b"ab";
    let span = Span { start: 0, end: 2 };
    let anchored = Anchored::Yes;
    let input = Input { haystack, span, anchored, earliest: false };
    let _ = Input::from(haystack);
}

#[test]
fn test_input_from_haystack_with_valid_utf8() {
    let haystack: &[u8] = b"hello";
    let span = Span { start: 0, end: 5 };
    let anchored = Anchored::Pattern(PatternID(1));
    let input = Input { haystack, span, anchored, earliest: false };
    let _ = Input::from(haystack);
}

#[test]
#[should_panic]
fn test_input_from_haystack_with_out_of_bounds_span() {
    let haystack: &[u8] = b"test";
    let span = Span { start: 0, end: 5 };
    let anchored = Anchored::No;
    let input = Input { haystack, span, anchored, earliest: false };
    let _ = Input::from(haystack);
}

#[test]
#[should_panic]
fn test_input_from_haystack_with_invalid_span() {
    let haystack: &[u8] = b"data";
    let span = Span { start: 2, end: 1 };
    let anchored = Anchored::Yes;
    let input = Input { haystack, span, anchored, earliest: false };
    let _ = Input::from(haystack);
}

