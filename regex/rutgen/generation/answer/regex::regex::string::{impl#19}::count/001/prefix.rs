// Answer 0

#[test]
fn test_count_with_single_match() {
    let haystack = "abc def ghi";
    let it = meta::CapturesMatches::new(haystack, "abc");
    let capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_multiple_matches() {
    let haystack = "abc def ghi abc jkl";
    let it = meta::CapturesMatches::new(haystack, "abc");
    let capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_no_matches() {
    let haystack = "xyz";
    let it = meta::CapturesMatches::new(haystack, "abc");
    let capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_empty_haystack() {
    let haystack = "";
    let it = meta::CapturesMatches::new(haystack, "abc");
    let capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

