// Answer 0

#[test]
fn test_get_anchored_no() {
    let haystack: &[u8] = b"foobar";
    let span = Span { start: 0, end: 6 };
    let input = Input::new(haystack).span(span).anchored(Anchored::No).earliest(false);
    let result = input.get_anchored();
}

#[test]
fn test_get_anchored_yes() {
    let haystack: &[u8] = b"foobar";
    let span = Span { start: 0, end: 6 };
    let input = Input::new(haystack).span(span).anchored(Anchored::Yes).earliest(false);
    let result = input.get_anchored();
}

#[test]
fn test_get_anchored_pattern() {
    let haystack: &[u8] = b"foobar";
    let span = Span { start: 0, end: 6 };
    let pid = PatternID::must(5);
    let input = Input::new(haystack).span(span).anchored(Anchored::Pattern(pid)).earliest(false);
    let result = input.get_anchored();
}

#[test]
fn test_get_anchored_multiple_conditions() {
    let haystack: &[u8] = b"foobar";
    let span_no = Span { start: 0, end: 6 };
    let span_yes = Span { start: 1, end: 2 };
    let pid = PatternID::must(3);
    
    let input_no = Input::new(haystack).span(span_no).anchored(Anchored::No).earliest(false);
    let result_no = input_no.get_anchored();
    
    let input_yes = Input::new(haystack).span(span_yes).anchored(Anchored::Yes).earliest(true);
    let result_yes = input_yes.get_anchored();
    
    let input_pattern = Input::new(haystack).span(span_no).anchored(Anchored::Pattern(pid)).earliest(false);
    let result_pattern = input_pattern.get_anchored();
}

