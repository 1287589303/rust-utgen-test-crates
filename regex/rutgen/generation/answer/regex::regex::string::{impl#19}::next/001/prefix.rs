// Answer 0

#[test]
fn test_next_with_valid_haystack_and_captures() {
    let haystack = "abc123def";
    let pattern = regex::Regex::new(r"(\d+)").unwrap();
    let it = pattern.captures_iter(haystack);
    let mut capture_matches = regex::CaptureMatches {
        haystack,
        it: it.collect::<Vec<_>>().into_iter(),
    };

    let result = capture_matches.next();
}

#[test]
fn test_next_with_boundary_case_haystack() {
    let haystack = "123";
    let pattern = regex::Regex::new(r"(\d+)").unwrap();
    let it = pattern.captures_iter(haystack);
    let mut capture_matches = regex::CaptureMatches {
        haystack,
        it: it.collect::<Vec<_>>().into_iter(),
    };

    let result = capture_matches.next();
}

#[test]
fn test_next_with_multiple_matches() {
    let haystack = "abc 123 def 456";
    let pattern = regex::Regex::new(r"(\d+)").unwrap();
    let it = pattern.captures_iter(haystack);
    let mut capture_matches = regex::CaptureMatches {
        haystack,
        it: it.collect::<Vec<_>>().into_iter(),
    };

    let result1 = capture_matches.next();
    let result2 = capture_matches.next();
}

#[test]
fn test_next_with_no_matches() {
    let haystack = "abcdef";
    let pattern = regex::Regex::new(r"(\d+)").unwrap();
    let it = pattern.captures_iter(haystack);
    let mut capture_matches = regex::CaptureMatches {
        haystack,
        it: it.collect::<Vec<_>>().into_iter(),
    };

    let result = capture_matches.next();
}

#[test]
fn test_next_with_empty_haystack() {
    let haystack = "";
    let pattern = regex::Regex::new(r"(\d+)").unwrap();
    let it = pattern.captures_iter(haystack);
    let mut capture_matches = regex::CaptureMatches {
        haystack,
        it: it.collect::<Vec<_>>().into_iter(),
    };

    let result = capture_matches.next();
}

