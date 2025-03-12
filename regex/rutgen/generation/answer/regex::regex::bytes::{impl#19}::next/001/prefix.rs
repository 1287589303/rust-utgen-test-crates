// Answer 0

#[test]
fn test_next_with_matches() {
    let haystack: &[u8] = b"abc123";
    let pattern = regex_automata::RegularRegex::new(b"123").unwrap();
    let input = Input::new(haystack);
    let it = pattern.find_iter(input).unwrap();
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.next();
}

#[test]
fn test_next_with_no_matches() {
    let haystack: &[u8] = b"abc";
    let pattern = regex_automata::RegularRegex::new(b"123").unwrap();
    let input = Input::new(haystack);
    let it = pattern.find_iter(input).unwrap();
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.next();
}

#[test]
fn test_next_with_static_captures_len_zero() {
    let haystack: &[u8] = b"abc";
    let pattern = regex_automata::RegularRegex::new(b"abc").unwrap();
    let input = Input::new(haystack);
    let it = pattern.find_iter(input).unwrap();
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.next();
}

#[test]
fn test_next_with_large_static_captures_len() {
    let haystack: &[u8] = b"abcabcabc";
    let pattern = regex_automata::RegularRegex::new(b"abc").unwrap();
    let input = Input::new(haystack);
    let it = pattern.find_iter(input).unwrap();
    let mut capture_matches = CaptureMatches { haystack, it };
    let result = capture_matches.next();
}

