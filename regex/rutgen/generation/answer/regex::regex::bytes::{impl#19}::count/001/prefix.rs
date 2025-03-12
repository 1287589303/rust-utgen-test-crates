// Answer 0

#[test]
fn test_count_with_empty_haystack() {
    let haystack: &[u8] = b"";
    let pattern = b"abc"; // arbitrary pattern
    let it = meta::CapturesMatches::new(haystack, pattern);
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_single_byte_haystack() {
    let haystack: &[u8] = b"a";
    let pattern = b"a"; // pattern matches the single byte
    let it = meta::CapturesMatches::new(haystack, pattern);
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_no_matches() {
    let haystack: &[u8] = b"def";
    let pattern = b"abc"; // arbitrary pattern that doesn't match
    let it = meta::CapturesMatches::new(haystack, pattern);
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_multiple_matches() {
    let haystack: &[u8] = b"ababab";
    let pattern = b"ab"; // pattern matches multiple times
    let it = meta::CapturesMatches::new(haystack, pattern);
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_pattern_at_start() {
    let haystack: &[u8] = b"abcde";
    let pattern = b"abc"; // pattern matches at the start
    let it = meta::CapturesMatches::new(haystack, pattern);
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_pattern_at_middle() {
    let haystack: &[u8] = b"xyzabcxyz";
    let pattern = b"abc"; // pattern matches in the middle
    let it = meta::CapturesMatches::new(haystack, pattern);
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_pattern_at_end() {
    let haystack: &[u8] = b"defabc";
    let pattern = b"abc"; // pattern matches at the end
    let it = meta::CapturesMatches::new(haystack, pattern);
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

#[test]
fn test_count_with_large_haystack() {
    let haystack: &[u8] = b"abcabcabcabcabcabcabcabcabc"; // large haystack
    let pattern = b"abc"; // pattern matches multiple times
    let it = meta::CapturesMatches::new(haystack, pattern);
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.count();
}

