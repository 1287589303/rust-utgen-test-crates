// Answer 0

#[test]
fn test_size_hint_empty_pattern_no_captures() {
    let haystack = "abcde";
    let pattern: &str = ""; // empty pattern
    let it = captures::CapturesPatternIter::new(haystack, pattern);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    sub_capture_matches.size_hint();
}

#[test]
fn test_size_hint_single_character_pattern_single_capture() {
    let haystack = "abcde";
    let pattern: &str = "a"; // single character pattern
    let it = captures::CapturesPatternIter::new(haystack, pattern);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    sub_capture_matches.size_hint();
}

#[test]
fn test_size_hint_multi_character_pattern_single_capture() {
    let haystack = "abcde";
    let pattern: &str = "abc"; // multi-character pattern
    let it = captures::CapturesPatternIter::new(haystack, pattern);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    sub_capture_matches.size_hint();
}

#[test]
fn test_size_hint_single_character_pattern_multiple_captures() {
    let haystack = "aaaa";
    let pattern: &str = "a"; // single character pattern with multiple captures
    let it = captures::CapturesPatternIter::new(haystack, pattern);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    sub_capture_matches.size_hint();
}

#[test]
fn test_size_hint_multi_character_pattern_multiple_captures() {
    let haystack = "abcabcabc";
    let pattern: &str = "abc"; // multi-character pattern repeated
    let it = captures::CapturesPatternIter::new(haystack, pattern);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    sub_capture_matches.size_hint();
}

#[test]
fn test_size_hint_no_captures_large_haystack() {
    let haystack = "xyzzyx";
    let pattern: &str = "abc"; // no captures
    let it = captures::CapturesPatternIter::new(haystack, pattern);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    sub_capture_matches.size_hint();
}

