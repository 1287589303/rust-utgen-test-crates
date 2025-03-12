// Answer 0

#[test]
fn count_empty_captures() {
    let haystack: &[u8] = b"";
    let it = captures::CapturesPatternIter::new(&[], haystack);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    let count = sub_capture_matches.count();
}

#[test]
fn count_single_match() {
    let haystack: &[u8] = b"abc";
    let it = captures::CapturesPatternIter::new(&[0..3], haystack);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    let count = sub_capture_matches.count();
}

#[test]
fn count_multiple_matches() {
    let haystack: &[u8] = b"abcabc";
    let it = captures::CapturesPatternIter::new(&[0..3, 3..6], haystack);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    let count = sub_capture_matches.count();
}

#[test]
fn count_no_matches() {
    let haystack: &[u8] = b"abc";
    let it = captures::CapturesPatternIter::new(&[], haystack);
    let sub_capture_matches = SubCaptureMatches { haystack, it };
    let count = sub_capture_matches.count();
}

#[test]
fn count_boundary_case() {
    let haystack = vec![0u8; 10000]; // 10,000 bytes
    let it = captures::CapturesPatternIter::new(&[0..10000], &haystack);
    let sub_capture_matches = SubCaptureMatches { haystack: &haystack, it };
    let count = sub_capture_matches.count();
}

