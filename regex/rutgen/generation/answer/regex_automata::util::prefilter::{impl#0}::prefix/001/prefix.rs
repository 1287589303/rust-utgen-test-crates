// Answer 0

#[test]
fn test_prefix_match_valid() {
    let haystack = b"Hello Bruce Springsteen!";
    let span = Span { start: 6, end: 12 }; // "Bruce"
    
    let pre = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &syntax::parse(r"Bruce \w+").unwrap()).expect("a prefilter");
    let result = pre.prefix(haystack, span);
}

#[test]
fn test_prefix_no_match_at_start() {
    let haystack = b"Hello Bruce Springsteen!";
    let span = Span { start: 0, end: haystack.len() }; // "Hello"
    
    let pre = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &syntax::parse(r"Bruce \w+").unwrap()).expect("a prefilter");
    let result = pre.prefix(haystack, span);
}

#[test]
fn test_prefix_no_match_empty_haystack() {
    let haystack = b"";
    let span = Span { start: 0, end: 0 }; // empty span
    
    let pre = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &syntax::parse(r"Bruce \w+").unwrap()).expect("a prefilter");
    let result = pre.prefix(haystack, span);
}

#[test]
fn test_prefix_no_match_different_content() {
    let haystack = b"Hello World!";
    let span = Span { start: 0, end: haystack.len() }; // "Hello World!"
    
    let pre = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &syntax::parse(r"Bruce \w+").unwrap()).expect("a prefilter");
    let result = pre.prefix(haystack, span);
}

#[test]
fn test_prefix_match_multiple_valid() {
    let haystack = b"Hello Bruce Springsteen and Bruce Wayne!";
    let span = Span { start: 6, end: 12 }; // "Bruce" first occurrence
    
    let pre = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &syntax::parse(r"Bruce \w+").unwrap()).expect("a prefilter");
    let result = pre.prefix(haystack, span);
}

#[test]
fn test_prefix_beyond_haystack_length() {
    let haystack = b"Hello Bruce Springsteen!";
    let span = Span { start: 6, end: 30 }; // span exceeds haystack length
    
    let pre = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &syntax::parse(r"Bruce \w+").unwrap()).expect("a prefilter");
    let result = pre.prefix(haystack, span);
}

