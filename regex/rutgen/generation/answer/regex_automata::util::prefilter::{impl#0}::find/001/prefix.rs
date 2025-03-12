// Answer 0

#[test]
fn test_find_single_byte_match() {
    let needles = vec![b"e"];
    let haystack = b"Hello";
    let span = Span { start: 0, end: haystack.len() };
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, &needles).expect("Failed to create prefilter");
    prefilter.find(haystack, span);
}

#[test]
fn test_find_multiple_byte_match() {
    let needles = vec![b"Bruce"];
    let haystack = b"Hello Bruce Springsteen!";
    let span = Span { start: 6, end: 30 };
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, &needles).expect("Failed to create prefilter");
    prefilter.find(haystack, span);
}

#[test]
fn test_find_prefix_in_middle() {
    let needles = vec![b"Hello"];
    let haystack = b"Hello Bruce Springsteen!";
    let span = Span { start: 0, end: 30 };
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, &needles).expect("Failed to create prefilter");
    prefilter.find(haystack, span);
}

#[test]
fn test_find_no_match() {
    let needles = vec![b"notfound"];
    let haystack = b"Hello Bruce Springsteen!";
    let span = Span { start: 0, end: 30 };
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, &needles).expect("Failed to create prefilter");
    prefilter.find(haystack, span);
}

#[test]
fn test_find_with_large_haystack() {
    let needles = vec![b"pattern"];
    let haystack = b"".repeat(1000) + b"pattern" + b" more text.";
    let span = Span { start: 0, end: haystack.len() };
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, &needles).expect("Failed to create prefilter");
    prefilter.find(haystack.as_bytes(), span);
}

#[test]
fn test_find_empty_haystack() {
    let needles = vec![b"needle"];
    let haystack = b"";
    let span = Span { start: 0, end: 0 };
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, &needles).expect("Failed to create prefilter");
    prefilter.find(haystack, span);
}

