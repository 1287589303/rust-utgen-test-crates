// Answer 0

#[test]
fn test_next_with_valid_data() {
    let haystack: &[u8] = b"hello world";
    let captures = vec![(0, 5), (6, 11)]; // Valid capture ranges
    let captures_iter = captures.iter().map(|&(start, end)| Some(captures::Capture { start, end }));
    let sub_capture_matches = SubCaptureMatches {
        haystack,
        it: captures_iter,
    };
    
    let _ = sub_capture_matches.next();
}

#[test]
fn test_next_with_boundary_capture() {
    let haystack: &[u8] = b"boundary test";
    let captures = vec![(0, 8), (8, 12)]; // Valid boundary capture ranges including bounds of the haystack
    let captures_iter = captures.iter().map(|&(start, end)| Some(captures::Capture { start, end }));
    let sub_capture_matches = SubCaptureMatches {
        haystack,
        it: captures_iter,
    };

    let _ = sub_capture_matches.next();
}

#[test]
fn test_next_with_single_capture() {
    let haystack: &[u8] = b"single";
    let captures = vec![(0, 7)]; // A single capture covering the entire haystack
    let captures_iter = captures.iter().map(|&(start, end)| Some(captures::Capture { start, end }));
    let sub_capture_matches = SubCaptureMatches {
        haystack,
        it: captures_iter,
    };

    let _ = sub_capture_matches.next();
}

#[test]
fn test_next_with_empty_captures() {
    let haystack: &[u8] = b"test";
    let captures: Vec<(usize, usize)> = vec![]; // No captures
    let captures_iter = captures.iter().map(|&_| None);
    let sub_capture_matches = SubCaptureMatches {
        haystack,
        it: captures_iter,
    };

    let _ = sub_capture_matches.next();
}

#[test]
fn test_next_with_captures_exceeding_haystack() {
    let haystack: &[u8] = b"check";
    let captures = vec![(2, 7)]; // Invalid capture (end exceeds haystack length)
    let captures_iter = captures.iter().map(|&(start, end)| Some(captures::Capture { start, end }));
    let sub_capture_matches = SubCaptureMatches {
        haystack,
        it: captures_iter,
    };

    let _ = sub_capture_matches.next();
}

