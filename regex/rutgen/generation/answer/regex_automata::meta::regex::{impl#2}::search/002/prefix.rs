// Answer 0

#[test]
fn test_search_with_matching_pattern() {
    let regex = Regex::new(r"Samwise|Sam").unwrap();
    let haystack = b"one of the chief characters, Samwise the Brave";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::No,
        earliest: false,
    };
    let _result = regex.search(&input);
}

#[test]
fn test_search_with_anchored_start() {
    let regex = Regex::new(r"^Samwise").unwrap();
    let haystack = b"Samwise the Brave";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::Yes,
        earliest: true,
    };
    let _result = regex.search(&input);
}

#[test]
fn test_search_with_minimum_length() {
    let regex = Regex::new(r"^S[a-z]+").unwrap();
    let haystack = b"Samwise the Brave";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::Yes,
        earliest: false,
    };
    let _result = regex.search(&input);
}

#[test]
fn test_search_with_edge_case_haystack() {
    let regex = Regex::new(r"Sam").unwrap();
    let haystack = b"Sam";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::No,
        earliest: true,
    };
    let _result = regex.search(&input);
}

#[test]
fn test_search_with_non_matching_pattern() {
    let regex = Regex::new(r"not_found").unwrap();
    let haystack = b"Samwise the Brave";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::No,
        earliest: false,
    };
    let _result = regex.search(&input);
}

